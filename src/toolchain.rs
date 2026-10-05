use std::io::{Write as _, stderr};
use std::path::Path;
use std::process::Command;

use anyhow::{Context as _, Result};
use cargo_metadata::semver::Version;

pub(crate) fn warn_if_older(root: &Path) -> Result<()> {
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
    let preset = Version::parse(env!("CARGO_PKG_VERSION"))?;

    if (rustc.major, rustc.minor) < (preset.major, preset.minor) {
        writeln!(
            stderr(),
            "warning: rustc {rustc} is older than Rust {}.{}, which the preset targets; \
             lints it does not know will trip `unknown_lints`",
            preset.major,
            preset.minor,
        )?;
    }
    Ok(())
}
