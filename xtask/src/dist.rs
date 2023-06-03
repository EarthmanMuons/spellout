#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]

use std::path::PathBuf;
use std::{env, fs};

use anyhow::Result;
use nanoserde::DeJson;
use xshell::Shell;

use crate::commands::cargo_cmd;
use crate::utils::{copy_dir_to, project_root, verbose_cd};
use crate::Config;

#[derive(Debug, DeJson)]
struct CheckMessage {
    out_dir: Option<String>,
    package_id: String,
    reason: String,
}

#[derive(Debug, DeJson)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Debug, DeJson)]
struct Package {
    #[allow(dead_code)]
    name: String,
    targets: Vec<Target>,
}

#[derive(Debug, DeJson)]
struct Target {
    kind: Vec<String>,
    name: String,
}

pub fn dist(config: &Config) -> Result<()> {
    if dist_dir().exists() {
        fs::remove_dir_all(dist_dir())?;
    }

    dist_binary(config)?;
    dist_build_script_outputs(config)?;
    // TODO: dist_docs()?;

    Ok(())
}

fn dist_binary(config: &Config) -> Result<()> {
    let sh = Shell::new()?;
    verbose_cd(&sh, project_root());

    let binaries = project_binaries(config)?;

    for binary in &binaries {
        let cmd_option = cargo_cmd(config, &sh);
        if let Some(cmd) = cmd_option {
            let args = vec!["build", "--profile", "production", "--bin", binary];
            cmd.args(args).run()?;
        }

        let binary_filename = if cfg!(target_os = "windows") {
            format!("{binary}.exe")
        } else {
            binary.to_string()
        };
        let src = production_dir().join(&binary_filename);

        // Destination: target/dist/binary/binary_filename
        let dest_dir = dist_dir().join(binary);
        fs::create_dir_all(&dest_dir)?;
        let dest = dest_dir.join(&binary_filename);

        eprintln!("Copying {} to {}", src.display(), dest.display());
        fs::copy(&src, &dest)?;
    }

    Ok(())
}

fn dist_build_script_outputs(config: &Config) -> Result<()> {
    let sh = Shell::new()?;
    verbose_cd(&sh, project_root());

    let binaries = project_binaries(config)?;

    for binary in &binaries {
        let cmd_option = cargo_cmd(config, &sh);
        if let Some(cmd) = cmd_option {
            eprintln!(
                "$ cargo check --profile production --message-format=json --quiet --bin {binary}"
            );
            let args = vec![
                "check",
                "--profile",
                "production",
                "--message-format=json",
                "--quiet",
                "--bin",
                binary,
            ];
            let output = cmd.args(args).output()?;

            let check_json = String::from_utf8(output.stdout)?;
            let out_dir_option = get_out_dir(&check_json, binary)?;

            if let Some(out_dir) = out_dir_option {
                let absolute_out_dir = PathBuf::from(out_dir);
                let src_dir = absolute_out_dir.strip_prefix(sh.current_dir())?;

                // Destination: target/dist/binary/
                let dest_dir = dist_dir().join(binary);
                fs::create_dir_all(&dest_dir)?;

                eprintln!("Copying {}/* to {}/", src_dir.display(), dest_dir.display());
                copy_dir_to(src_dir, &dest_dir)?;
            };
        }
    }

    Ok(())
}

fn project_binaries(config: &Config) -> Result<Vec<String>> {
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
                    // eprintln!("{package:?}");
                    binaries.push(target.name.clone());
                }
            }
        }
    }

    Ok(binaries)
}

fn get_out_dir(check_json: &str, binary: &str) -> Result<Option<String>> {
    for line in check_json.lines() {
        let msg: CheckMessage = DeJson::deserialize_json(line)?;
        if msg.reason == "build-script-executed"
            && msg.package_id.starts_with(binary)
            && msg.out_dir.is_some()
        {
            return Ok(msg.out_dir);
        }
    }
    Ok(None)
}

fn dist_dir() -> PathBuf {
    target_dir().join("dist")
}

fn production_dir() -> PathBuf {
    target_dir().join("production")
}

fn target_dir() -> PathBuf {
    let relative_path = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    PathBuf::from(relative_path)
}
