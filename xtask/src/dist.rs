use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

use anyhow::Result;
use nanoserde::DeJson;
use xshell::{cmd, Shell};

use crate::commands::cargo_cmd;
use crate::utils::project_root;
use crate::Config;

#[derive(Debug, DeJson)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Debug, DeJson)]
struct Package {
    #[allow(dead_code)]
    name: String,
    targets: Vec<Target>,
    version: String,
}

#[derive(Debug, DeJson)]
struct Target {
    kind: Vec<String>,
    name: String,
}

pub fn dist(config: &Config) -> Result<()> {
    env::set_current_dir(project_root())?;
    if let Some(target) = &config.target {
        env::set_var("CARGO_BUILD_TARGET", target);
    }

    if dist_dir().exists() {
        fs::remove_dir_all(dist_dir())?;
    }
    let binaries = project_binaries(config)?;

    for (binary, version) in &binaries {
        let dest_dir = dist_dir().join(format!("{binary}-{version}"));
        fs::create_dir_all(&dest_dir)?;

        build_binary(config, binary, &dest_dir)?;
        copy_docs(&dest_dir)?;
        generate_assets(config, binary, &dest_dir)?;
        create_archive(binary, version)?;
    }
    Ok(())
}

fn build_binary(config: &Config, binary: &str, dest_dir: &Path) -> Result<()> {
    let sh = Shell::new()?;

    let cmd_option = cargo_cmd(config, &sh);
    if let Some(cmd) = cmd_option {
        let args = vec!["build", "--release", "--bin", binary];
        cmd.args(args).run()?;
    }

    let binary_filename = if cfg!(target_os = "windows") {
        format!("{binary}.exe")
    } else {
        binary.to_string()
    };
    let src = release_dir().join(&binary_filename);
    let dest = dest_dir.join(&binary_filename);

    fs::copy(&src, &dest)?;
    eprintln!("Copied {} to {}", src.display(), dest.display());

    Ok(())
}

fn copy_docs(dest_dir: &Path) -> Result<()> {
    for file in [
        "CHANGELOG.md",
        "COPYING",
        "LICENSE",
        "LICENSE-APACHE",
        "LICENSE-MIT",
        "NOTICE",
        "README.md",
        "UNLICENSE",
    ] {
        let src = PathBuf::from(file);
        if src.exists() {
            let dest = dest_dir.join(file);

            fs::copy(&src, &dest)?;
            eprintln!("Copied {} to {}", src.display(), dest.display());
        }
    }
    Ok(())
}

fn generate_assets(config: &Config, binary: &str, dest_dir: &Path) -> Result<()> {
    let assets: HashMap<&str, (String, String)> = [
        ("man-page", ("man".to_string(), format!("{binary}.1"))),
        (
            "bash",
            ("completions".to_string(), format!("{binary}.bash")),
        ),
        (
            "elvish",
            ("completions".to_string(), format!("{binary}.elv")),
        ),
        (
            "fish",
            ("completions".to_string(), format!("{binary}.fish")),
        ),
        (
            "powershell",
            ("completions".to_string(), format!("_{binary}.ps1")),
        ),
        ("zsh", ("completions".to_string(), format!("_{binary}"))),
    ]
    .iter()
    .cloned()
    .collect();

    let sh = Shell::new()?;

    for (asset, (directory, filename)) in &assets {
        let cmd_option = cargo_cmd(config, &sh);
        if let Some(cmd) = cmd_option {
            let args = vec![
                "run",
                "--release",
                "--bin",
                binary,
                "--",
                "--generate",
                asset,
            ];
            let output = cmd.args(args).output()?;

            fs::create_dir_all(dest_dir.join(directory))?;
            fs::write(dest_dir.join(directory).join(filename), output.stdout)?;

            eprintln!("Generated {}/{directory}/{filename}", dest_dir.display());
        }
    }
    Ok(())
}

fn create_archive(binary: &str, version: &str) -> Result<()> {
    let sh = Shell::new()?;
    let temp_dir = sh.create_temp_dir()?;

    let input_dir = dist_dir();
    let output_filename = match env::consts::OS {
        "linux" | "macos" => format!("{binary}-{version}{}.tar.gz", target_triple()),
        "windows" => format!("{binary}-{version}{}.zip", target_triple()),
        _ => anyhow::bail!("Unsupported OS"),
    };
    let temp_output = temp_dir.path().join(output_filename.clone());

    match env::consts::OS {
        "linux" | "macos" => {
            // On Unix, create a tarball
            cmd!(sh, "tar -czf {temp_output} -C {input_dir} .").run()?;
        }
        "windows" => {
            // On Windows, create a zip file
            #[rustfmt::skip]
            cmd!(
                sh,
                "powershell.exe Compress-Archive -Path {input_dir}/* -DestinationPath {temp_output}"
            )
            .run()?;
        }
        _ => anyhow::bail!("Unsupported OS"),
    };

    let final_output = dist_dir().join(output_filename);
    eprintln!("$ mv {} {}", temp_output.display(), final_output.display());
    fs::copy(&temp_output, &final_output)?;
    fs::remove_file(temp_output)?;

    Ok(())
}

fn target_triple() -> String {
    env::var_os("CARGO_BUILD_TARGET").map_or_else(String::new, |target| {
        format!("-{}", target.to_string_lossy())
    })
}

fn project_binaries(config: &Config) -> Result<Vec<(String, String)>> {
    let sh = Shell::new()?;
    let mut binaries = Vec::new();

    let cmd_option = cargo_cmd(config, &sh);
    if let Some(cmd) = cmd_option {
        let args = vec!["metadata", "--no-deps", "--format-version=1"];
        let output = cmd.args(args).output()?;

        let metadata_json = String::from_utf8(output.stdout)?;
        let metadata: Metadata = DeJson::deserialize_json(&metadata_json)?;

        for package in metadata.packages {
            for target in &package.targets {
                if target.name != "xtask" && target.kind.contains(&"bin".to_string()) {
                    binaries.push((target.name.clone(), package.version.clone()));
                }
            }
        }
    }
    Ok(binaries)
}

fn dist_dir() -> PathBuf {
    target_dir().join("dist")
}

fn release_dir() -> PathBuf {
    env::var_os("CARGO_BUILD_TARGET").map_or_else(
        || target_dir().join("release"),
        |build_target| target_dir().join(build_target).join("release"),
    )
}

fn target_dir() -> PathBuf {
    let relative_path = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    PathBuf::from(relative_path)
}
