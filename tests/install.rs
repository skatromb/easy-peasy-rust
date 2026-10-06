//! Runs `cargo easy-peasy` on throwaway crates and checks what it writes.
#![cfg(test)]

use std::fs;
use std::process::{Command, Output, Stdio};

use rexpect::session::spawn_command;
use tempfile::TempDir;

const BIN: &str = env!("CARGO_BIN_EXE_cargo-easy-peasy");
const SETTINGS: &str = include_str!("../rules/clippy.toml");
const LIB: &str = "//! Fixture.\n";
const VIRTUAL: &str = "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"3\"\n";
const INHERITED: &str = "\n[lints]\nworkspace = true\n";
const UNWRAP_ALLOWED: &str = "\n[lints.clippy]\nunwrap_used = \"allow\"\n";

fn package(name: &str) -> String {
    format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n")
}

fn project(manifest: &str) -> TempDir {
    let dir = tempfile::tempdir().unwrap();
    write(&dir, "Cargo.toml", manifest);
    write(&dir, "src/lib.rs", LIB);
    dir
}

fn member(dir: &TempDir, name: &str, manifest: &str) {
    write(dir, &format!("crates/{name}/Cargo.toml"), manifest);
    write(dir, &format!("crates/{name}/src/lib.rs"), LIB);
}

fn write(dir: &TempDir, name: &str, text: &str) {
    let path = dir.path().join(name);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

fn read(dir: &TempDir, name: &str) -> String {
    fs::read_to_string(dir.path().join(name)).unwrap()
}

fn install(dir: &TempDir, flags: &[&str]) -> Output {
    Command::new(BIN)
        .arg("easy-peasy")
        .arg(dir.path())
        .args(flags)
        .stdin(Stdio::null())
        .output()
        .unwrap()
}

fn clippy_is_silent(dir: &TempDir) -> bool {
    let output = Command::new(env!("CARGO"))
        .args(["clippy", "--quiet", "--workspace", "--all-targets"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    output.status.success() && output.stderr.is_empty()
}

#[test]
fn refuses_to_run_without_a_terminal() {
    let dir = project(&package("solo"));

    let output = install(&dir, &[]);

    assert!(!output.status.success());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("--overwrite")
    );
    assert_eq!(read(&dir, "Cargo.toml"), package("solo"));
    assert!(!dir.path().join("clippy.toml").exists());
}

#[test]
fn installs_into_a_crate() {
    let dir = project(&package("solo"));

    assert!(install(&dir, &["-y"]).status.success());

    let cargo_toml = read(&dir, "Cargo.toml");
    assert!(cargo_toml.contains("\n[lints.clippy]\n"));
    assert!(cargo_toml.contains("\n[lints.rust]\n"));
    assert_eq!(read(&dir, "clippy.toml"), SETTINGS);
    assert!(clippy_is_silent(&dir));
}

#[test]
fn installs_into_a_virtual_workspace() {
    let dir = project(VIRTUAL);
    member(&dir, "one", &package("one"));
    member(&dir, "two", &format!("{}{UNWRAP_ALLOWED}", package("two")));

    assert!(install(&dir, &["-y"]).status.success());

    let cargo_toml = read(&dir, "Cargo.toml");
    assert!(cargo_toml.contains("\n[workspace.lints.clippy]\n"));
    assert!(!cargo_toml.contains("\n[lints"));
    for name in ["one", "two"] {
        let member_toml = read(&dir, &format!("crates/{name}/Cargo.toml"));
        assert_eq!(member_toml, format!("{}{INHERITED}", package(name)));
    }
    assert!(clippy_is_silent(&dir));
}

#[test]
fn installs_into_a_root_package() {
    let dir = project(&format!("{}\n[workspace]\n", package("root")));

    assert!(install(&dir, &["-y"]).status.success());

    let cargo_toml = read(&dir, "Cargo.toml");
    assert!(cargo_toml.contains("\n[workspace.lints.clippy]\n"));
    assert!(cargo_toml.ends_with(INHERITED));
    assert!(clippy_is_silent(&dir));
}

#[test]
fn keeps_and_lists_lints_the_preset_does_not_set() {
    let ours = "\n[lints.clippy]\n# ours\ndbg_macro = \"allow\"\n";
    let dir = project(&format!("{}{ours}", package("solo")));

    let output = install(&dir, &["-y"]);

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("clippy::dbg_macro: yours \"allow\", preset not set\n"));
    assert!(stderr.contains("--drop-existing"));
    assert!(read(&dir, "Cargo.toml").contains(ours));
}

#[test]
fn drops_lints_the_preset_does_not_set_with_the_flag() {
    let ours = "\n[lints.rustdoc]\nbroken_intra_doc_links = \"deny\"\n";
    let extra = "\n[lints.rust]\nelided_lifetimes_in_paths = \"warn\"\n";
    let dir = project(&format!("{}{ours}{extra}", package("solo")));

    let output = install(&dir, &["--drop-existing", "-y"]);

    assert_eq!(output.stderr, b"");
    let cargo_toml = read(&dir, "Cargo.toml");
    assert!(cargo_toml.contains(ours));
    assert!(!cargo_toml.contains("elided_lifetimes_in_paths"));
}

#[test]
fn overwrites_every_conflict_with_the_flag() {
    let dir = project(&format!("{}{UNWRAP_ALLOWED}", package("solo")));

    let output = install(&dir, &["--overwrite"]);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("clippy::unwrap_used: yours \"allow\", preset \"deny\"\n"));
    assert!(stdout.contains("Replaced yours\n"));
    assert!(read(&dir, "Cargo.toml").contains("unwrap_used = \"deny\"\n"));
}

#[test]
fn asks_about_each_conflict() {
    let dir = project(&format!("{}{UNWRAP_ALLOWED}", package("solo")));
    write(&dir, "clippy.toml", "too-many-lines-threshold = 50\n");
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
    assert!(read(&dir, "Cargo.toml").contains("unwrap_used = \"allow\"\n"));
    assert!(read(&dir, "clippy.toml").starts_with("too-many-lines-threshold = 25\n"));
}

#[test]
fn merges_into_a_hidden_clippy_toml() {
    let dir = project(&package("solo"));
    write(&dir, ".clippy.toml", "");

    assert!(install(&dir, &["-y"]).status.success());

    assert_eq!(read(&dir, ".clippy.toml"), SETTINGS);
    assert!(!dir.path().join("clippy.toml").exists());
}

#[test]
fn second_run_changes_nothing() {
    let dir = project(VIRTUAL);
    member(&dir, "one", &package("one"));
    assert!(install(&dir, &["-y"]).status.success());

    let output = install(&dir, &["-y"]);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.matches("Unchanged").count(), 3);
    assert!(!stdout.contains("Updated"));
}
