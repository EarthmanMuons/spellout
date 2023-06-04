#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]

use std::env;
use std::process::Command;

fn main() {
    let pkg_version = env::var("CARGO_PKG_VERSION").unwrap();

    current_git_hash().map_or_else(
        || {
            println!("cargo:rustc-env=SPELLOUT_VERSION={pkg_version}");
        },
        |hash| {
            println!("cargo:rustc-env=SPELLOUT_VERSION={pkg_version}+{hash}");
        },
    );
}

fn current_git_hash() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .output()
        .ok()?;

    let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if hash.is_empty() {
        return None;
    }

    let is_dirty = Command::new("git")
        .args(["diff", "--quiet", "HEAD", "--"])
        .status()
        .map(|status| !status.success())
        .unwrap_or(false);

    if is_dirty {
        Some(format!("{hash}.dirty"))
    } else {
        Some(hash)
    }
}
