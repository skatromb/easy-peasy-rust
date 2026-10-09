use std::io::{Write as _, stderr};
use std::path::Path;
use std::process::Command;

use anyhow::{Context as _, Result};
use cargo_metadata::semver::Version;

const CARGO_LINTS: Version = Version::new(1, 74, 0);

pub(crate) fn rust_release(root: &Path) -> Result<Version> {
    let output = Command::new("rustc")
        .arg("--version")
        .current_dir(root)
        .output()
        .context("running rustc --version")?;
    let banner = String::from_utf8(output.stdout)?;
    let version = banner
        .split_whitespace()
        .nth(1)
        .with_context(|| format!("reading the rustc version from `{banner}`"))?;
    let rustc = Version::parse(version)?;
    Ok(Version::new(rustc.major, rustc.minor, 0))
}

pub(crate) fn reads_lints(rust: &Version) -> Result<bool> {
    let reads = *rust >= CARGO_LINTS;

    if !reads {
        writeln!(
            stderr(),
            "warning: skipped the lints, Cargo reads them from Rust {CARGO_LINTS}, upgrade it and rerun to add them"
        )?;
    }
    Ok(reads)
}
