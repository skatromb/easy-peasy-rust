use std::collections::HashSet;
use std::path::Path;
use std::process::Command;

use anyhow::{Context as _, Result};
use cargo_metadata::semver::Version;
use toml_edit::{DocumentMut, Table};

use crate::block::{Kind, warn};

const VALIDITY: &str = include_str!("../rules/validity.toml");
const CARGO_LINTS: Version = Version::new(1, 74, 0);

pub(crate) struct Toolchain {
    rust: Version,
    known: HashSet<String>,
}

impl Toolchain {
    pub(crate) fn detect(root: &Path) -> Result<Self> {
        let rust = rust_release(root)?;
        let known = known_by(&rust)?;
        Ok(Self { rust, known })
    }

    pub(crate) fn reads_lints(&self) -> bool {
        self.rust >= CARGO_LINTS
    }

    pub(crate) fn retain(&self, preset: &mut Table, kind: Kind) -> Vec<String> {
        let mut skipped = Vec::new();
        preset.retain(|name, _| {
            let known = self.known.contains(&format!("{}.{name}", kind.section()));
            if !known {
                skipped.push(kind.label(name));
            }
            known
        });
        skipped
    }

    pub(crate) fn warn_skipped(&self, mut skipped: Vec<String>) -> Result<()> {
        skipped.sort();
        if !self.reads_lints() {
            warn(format_args!(
                "skipped the lints, Cargo reads them from Rust {}, upgrade it and rerun to add them",
                release(&CARGO_LINTS)
            ))?;
        }
        if !skipped.is_empty() {
            warn(format_args!(
                "skipped what Rust {} does not know yet, upgrade it and rerun to add: {}",
                release(&self.rust),
                skipped.join(", ")
            ))?;
        }
        Ok(())
    }
}

fn rust_release(root: &Path) -> Result<Version> {
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

fn known_by(rust: &Version) -> Result<HashSet<String>> {
    let validity: DocumentMut = VALIDITY.parse()?;
    let mut known = HashSet::new();
    for (section, releases) in validity.iter() {
        let names = releases
            .as_table()
            .context("validity section is not a table")?;
        for (name, release) in names {
            let since = release.as_str().context("release is not a string")?;
            if Version::parse(&format!("{since}.0"))? <= *rust {
                let _ = known.insert(format!("{section}.{name}"));
            }
        }
    }
    Ok(known)
}

fn release(version: &Version) -> String {
    format!("{}.{}", version.major, version.minor)
}
