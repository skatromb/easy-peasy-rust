//! `cargo easy-peasy`: installs the easy-peasy-rust lint preset into a Cargo workspace.

mod merge;

use std::fs;
use std::io::{ErrorKind, Write, stdout};
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, ensure};
use clap::{Args, Parser};
use toml_edit::DocumentMut;

const LINTS: &str = include_str!("../preset/lints.toml");
const CLIPPY: &str = include_str!("../preset/clippy.toml");

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum Cargo {
    /// Install the easy-peasy-rust lint preset into a crate or workspace.
    EasyPeasy(Install),
}

#[derive(Args)]
struct Install {
    /// Crate or workspace root.
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Take the preset's value on every conflict.
    #[arg(short = 'y', long = "override")]
    take_preset: bool,
}

fn main() -> Result<()> {
    let Cargo::EasyPeasy(install) = Cargo::parse();
    let manifest = install.path.join("Cargo.toml");
    ensure!(
        manifest.is_file(),
        "no Cargo.toml in {}",
        install.path.display()
    );
    let lints: DocumentMut = LINTS.parse()?;
    let clippy: DocumentMut = CLIPPY.parse()?;
    let mut out = stdout().lock();
    let mut conflicts = update(&manifest, &mut out, |doc| {
        merge::lints(doc, &lints, install.take_preset)
    })?;
    conflicts.extend(update(
        &install.path.join("clippy.toml"),
        &mut out,
        |doc| Ok(merge::table(doc, &clippy, install.take_preset)),
    )?);
    report(&mut out, &conflicts, install.take_preset)
}

fn update(
    path: &Path,
    out: &mut impl Write,
    apply: impl FnOnce(&mut DocumentMut) -> Result<Vec<String>>,
) -> Result<Vec<String>> {
    let before = read(path)?;
    let mut doc: DocumentMut = before.as_deref().unwrap_or_default().parse()?;
    let conflicts = apply(&mut doc)?;
    let after = doc.to_string();
    let status = match before {
        None => "Created",
        Some(text) if text == after => "Unchanged",
        Some(_) => "Updated",
    };
    if status != "Unchanged" {
        fs::write(path, after).with_context(|| format!("writing {}", path.display()))?;
    }
    writeln!(out, "{status} {}", path.display())?;
    Ok(conflicts)
}

fn read(path: &Path) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("reading {}", path.display())),
    }
}

fn report(out: &mut impl Write, conflicts: &[String], take_preset: bool) -> Result<()> {
    let verdict = if take_preset {
        "Replaced yours"
    } else {
        "Kept yours"
    };
    for conflict in conflicts {
        writeln!(out, "{verdict}: {conflict}")?;
    }
    writeln!(
        out,
        "Run `cargo clippy --all-targets` to see what it flags."
    )?;
    Ok(())
}
