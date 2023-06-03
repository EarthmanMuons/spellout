use std::path::{Path, PathBuf};
use std::{env, fs};

use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;
use cli::Cli;

#[path = "src/cli.rs"]
mod cli;

type DynError = Box<dyn std::error::Error>;

fn main() -> Result<(), DynError> {
    // Cargo sets the `OUT_DIR` environment variable to the folder in which all
    // output and intermediate artifacts should be placed.
    let out_dir = match env::var("OUT_DIR") {
        Ok(val) => PathBuf::from(val),
        Err(e) => {
            eprintln!("OUT_DIR environment variable not defined");
            return Err(e.into());
        }
    };

    generate_completions(&out_dir)?;
    generate_man_page(&out_dir)?;

    Ok(())
}

fn generate_completions<P: AsRef<Path>>(out_dir: P) -> Result<(), DynError> {
    let completions_dir = out_dir.as_ref().join("completions");
    fs::create_dir_all(&completions_dir)?;

    // Assume the binary name matches the package name
    let binary_name = env::var("CARGO_PKG_NAME")?;

    let mut cmd = Cli::command();
    for shell in [
        Shell::Bash,
        Shell::Elvish,
        Shell::Fish,
        Shell::PowerShell,
        Shell::Zsh,
    ] {
        generate_to(shell, &mut cmd, &binary_name, &completions_dir)?;
    }

    Ok(())
}

fn generate_man_page<P: AsRef<Path>>(out_dir: P) -> Result<(), DynError> {
    let man_dir = out_dir.as_ref().join("man");
    fs::create_dir_all(&man_dir)?;

    // Assume the binary name matches the package name
    let binary_name = env::var("CARGO_PKG_NAME")?;

    let cmd = Cli::command();
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    fs::write(man_dir.join(format!("{binary_name}.1")), buffer)?;

    Ok(())
}
