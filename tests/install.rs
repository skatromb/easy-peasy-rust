//! Runs `cargo easy-peasy` on throwaway crates and checks what it writes.

use std::fs;
use std::process::{Command, Output, Stdio};

use anyhow::{Context as _, Result};
use rexpect::session::spawn_command;
use tempfile::TempDir;

const BIN: &str = env!("CARGO_BIN_EXE_cargo-easy-peasy");
const SETTINGS: &str = include_str!("../preset/clippy.toml");
const LIB: &str = "//! Fixture.\n";
const VIRTUAL: &str = "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"3\"\n";
const INHERITED: &str = "\n[lints]\nworkspace = true\n";
const UNWRAP_ALLOWED: &str = "\n[lints.clippy]\nunwrap_used = \"allow\"\n";

fn package(name: &str) -> String {
    format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n")
}

fn project(manifest: &str) -> Result<TempDir> {
    let dir = tempfile::tempdir()?;
    write(&dir, "Cargo.toml", manifest)?;
    write(&dir, "src/lib.rs", LIB)?;
    Ok(dir)
}

fn member(dir: &TempDir, name: &str, manifest: &str) -> Result<()> {
    write(dir, &format!("crates/{name}/Cargo.toml"), manifest)?;
    write(dir, &format!("crates/{name}/src/lib.rs"), LIB)
}

fn write(dir: &TempDir, name: &str, text: &str) -> Result<()> {
    let path = dir.path().join(name);
    fs::create_dir_all(path.parent().context("no parent directory")?)?;
    Ok(fs::write(path, text)?)
}

fn read(dir: &TempDir, name: &str) -> Result<String> {
    Ok(fs::read_to_string(dir.path().join(name))?)
}

fn install(dir: &TempDir, flags: &[&str]) -> Result<Output> {
    Ok(Command::new(BIN)
        .arg("easy-peasy")
        .arg(dir.path())
        .args(flags)
        .stdin(Stdio::null())
        .output()?)
}

fn clippy_passes(dir: &TempDir) -> Result<bool> {
    let status = Command::new(env!("CARGO"))
        .args(["clippy", "--quiet", "--workspace", "--all-targets"])
        .args(["--", "--deny", "warnings"])
        .current_dir(dir.path())
        .status()?;
    Ok(status.success())
}

#[test]
fn refuses_to_run_without_a_terminal() {
    let dir = project(&package("solo")).unwrap();

    let output = install(&dir, &[]).unwrap();

    assert!(!output.status.success());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("--overwrite")
    );
    assert_eq!(read(&dir, "Cargo.toml").unwrap(), package("solo"));
    assert!(!dir.path().join("clippy.toml").exists());
}

#[test]
fn installs_into_a_crate() {
    let dir = project(&package("solo")).unwrap();

    assert!(install(&dir, &["-y"]).unwrap().status.success());

    let cargo_toml = read(&dir, "Cargo.toml").unwrap();
    assert!(cargo_toml.contains("\n[lints.clippy]\n"));
    assert!(cargo_toml.contains("\n[lints.rust]\n"));
    assert_eq!(read(&dir, "clippy.toml").unwrap(), SETTINGS);
    assert!(clippy_passes(&dir).unwrap());
}

#[test]
fn installs_into_a_virtual_workspace() {
    let dir = project(VIRTUAL).unwrap();
    member(&dir, "one", &package("one")).unwrap();
    member(&dir, "two", &format!("{}{UNWRAP_ALLOWED}", package("two"))).unwrap();

    assert!(install(&dir, &["-y"]).unwrap().status.success());

    let cargo_toml = read(&dir, "Cargo.toml").unwrap();
    assert!(cargo_toml.contains("\n[workspace.lints.clippy]\n"));
    assert!(!cargo_toml.contains("\n[lints"));
    for name in ["one", "two"] {
        let member_toml = read(&dir, &format!("crates/{name}/Cargo.toml")).unwrap();
        assert_eq!(member_toml, format!("{}{INHERITED}", package(name)));
    }
    assert!(clippy_passes(&dir).unwrap());
}

#[test]
fn installs_into_a_root_package() {
    let dir = project(&format!("{}\n[workspace]\n", package("root"))).unwrap();

    assert!(install(&dir, &["-y"]).unwrap().status.success());

    let cargo_toml = read(&dir, "Cargo.toml").unwrap();
    assert!(cargo_toml.contains("\n[workspace.lints.clippy]\n"));
    assert!(cargo_toml.ends_with(INHERITED));
    assert!(clippy_passes(&dir).unwrap());
}

#[test]
fn keeps_what_the_preset_does_not_set() {
    let ours = "\n[lints.clippy]\n# ours\ndbg_macro = \"allow\"\n";
    let dir = project(&format!("{}{ours}", package("solo"))).unwrap();

    assert!(install(&dir, &["-y"]).unwrap().status.success());

    assert!(read(&dir, "Cargo.toml").unwrap().contains(ours));
}

#[test]
fn overwrites_every_conflict_with_the_flag() {
    let dir = project(&format!("{}{UNWRAP_ALLOWED}", package("solo"))).unwrap();

    let output = install(&dir, &["--overwrite"]).unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("clippy::unwrap_used: yours \"allow\", preset \"deny\"\n"));
    assert!(stdout.contains("Replaced yours\n"));
    assert!(
        read(&dir, "Cargo.toml")
            .unwrap()
            .contains("unwrap_used = \"deny\"\n")
    );
}

#[test]
fn asks_about_each_conflict() {
    let dir = project(&format!("{}{UNWRAP_ALLOWED}", package("solo"))).unwrap();
    write(&dir, "clippy.toml", "too-many-lines-threshold = 50\n").unwrap();
    let mut command = Command::new(BIN);
    let _ = command.arg("easy-peasy").arg(dir.path());

    let mut session = spawn_command(command, Some(30_000)).unwrap();
    let first = session.exp_string("[y/N] ").unwrap();
    let _ = session.send_line("").unwrap();
    let second = session.exp_string("[y/N] ").unwrap();
    let _ = session.send_line("y").unwrap();
    let rest = session.exp_eof().unwrap();

    assert!(first.contains("clippy::unwrap_used"));
    assert!(second.contains("too-many-lines-threshold"));
    assert!(rest.contains("Updated clippy.toml"));
    assert!(
        read(&dir, "Cargo.toml")
            .unwrap()
            .contains("unwrap_used = \"allow\"\n")
    );
    assert!(
        read(&dir, "clippy.toml")
            .unwrap()
            .starts_with("too-many-lines-threshold = 25\n")
    );
}

#[test]
fn merges_into_a_hidden_clippy_toml() {
    let dir = project(&package("solo")).unwrap();
    write(&dir, ".clippy.toml", "").unwrap();

    assert!(install(&dir, &["-y"]).unwrap().status.success());

    assert_eq!(read(&dir, ".clippy.toml").unwrap(), SETTINGS);
    assert!(!dir.path().join("clippy.toml").exists());
}

#[test]
fn second_run_changes_nothing() {
    let dir = project(VIRTUAL).unwrap();
    member(&dir, "one", &package("one")).unwrap();
    assert!(install(&dir, &["-y"]).unwrap().status.success());

    let output = install(&dir, &["-y"]).unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.matches("Unchanged").count(), 3);
    assert!(!stdout.contains("Updated"));
}
