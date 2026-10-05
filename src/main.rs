//! `cargo easy-peasy`: installs the easy-peasy-rust lint preset into a Cargo workspace.

mod merge;

use std::fs;
use std::io::{ErrorKind, IsTerminal as _, Write as _, stdin, stdout};
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, ensure};
use clap::Parser;
use merge::{Conflict, Kind};
use toml_edit::DocumentMut;

const LINTS: &str = include_str!("../preset/lints.toml");
const CLIPPY: &str = include_str!("../preset/clippy.toml");

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum Cargo {
    /// Install the easy-peasy-rust lint preset into a crate or workspace.
    EasyPeasy(Args),
}

#[derive(clap::Args)]
struct Args {
    /// Crate or workspace root.
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Take the preset's value on every conflict.
    #[arg(short = 'y', long = "override")]
    overwrite: bool,
}

fn main() -> Result<()> {
    let Cargo::EasyPeasy(args) = Cargo::parse();
    let manifest = args.path.join("Cargo.toml");
    ensure!(
        manifest.is_file(),
        "no Cargo.toml in {}",
        args.path.display()
    );
    let lints: DocumentMut = LINTS.parse()?;
    let clippy: DocumentMut = CLIPPY.parse()?;
    let mut decide = |conflict: &Conflict| resolve(conflict, args.overwrite);
    update(&manifest, |doc| merge::lints(doc, &lints, &mut decide))?;
    update(&args.path.join("clippy.toml"), |doc| {
        merge::table(doc, &clippy, Kind::ClippySetting, &mut decide)
    })?;
    writeln!(
        stdout(),
        "Run `cargo clippy --all-targets` to see what it flags."
    )?;
    Ok(())
}

fn resolve(conflict: &Conflict, overwrite: bool) -> Result<bool> {
    let mut out = stdout().lock();
    writeln!(out, "{conflict}")?;
    if overwrite {
        writeln!(out, "Replaced yours")?;
        return Ok(true);
    }
    if !stdin().is_terminal() {
        writeln!(out, "Kept yours")?;
        return Ok(false);
    }
    write!(out, "Take the preset's value? [y/N] ")?;
    out.flush()?;
    let answer = stdin().lines().next().transpose()?.unwrap_or_default();
    Ok(answer.trim().eq_ignore_ascii_case("y"))
}

fn update(path: &Path, apply: impl FnOnce(&mut DocumentMut) -> Result<()>) -> Result<()> {
    let before = read(path)?;
    let mut doc: DocumentMut = before.as_deref().unwrap_or_default().parse()?;
    apply(&mut doc)?;
    let after = doc.to_string();
    let status = match before {
        None => "Created",
        Some(text) if text == after => "Unchanged",
        Some(_) => "Updated",
    };
    if status != "Unchanged" {
        fs::write(path, after).with_context(|| format!("writing {}", path.display()))?;
    }
    writeln!(stdout(), "{status} {}", path.display())?;
    Ok(())
}

fn read(path: &Path) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("reading {}", path.display())),
    }
}
