use std::fs;
use std::process::Command;

use super::{BIN, clippy_is_silent, fixture, install, project, read};

const INHERITED: &str = "\n[lints]\nworkspace = true\n";

#[test]
fn installs_into_a_virtual_workspace_from_a_member() {
    let dir = project("workspace");

    let output = Command::new(BIN)
        .args(["easy-peasy", "-y"])
        .current_dir(dir.path().join("crates/one"))
        .output()
        .unwrap();

    assert!(output.status.success());
    let cargo_toml = read(&dir, "Cargo.toml");
    assert!(cargo_toml.contains("\n[workspace.lints.clippy]\n"));
    assert!(!cargo_toml.contains("\n[lints"));
    assert!(read(&dir, "crates/one/Cargo.toml").ends_with(INHERITED));
    assert_eq!(
        read(&dir, "crates/two/Cargo.toml"),
        fixture("workspace/crates/two/Cargo.toml")
    );
    assert!(clippy_is_silent(&dir));
}

#[test]
fn lists_every_workspace_member_in_one_block() {
    let dir = project("workspace");

    let output = install(&dir, &["--diff"]);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.matches("Workspace lints").count(), 1);
    assert!(stdout.contains("\n  crates/one/Cargo.toml\n"));
    assert!(stdout.contains("\n  crates/two/Cargo.toml: { clippy = { unwrap_used = \"allow\" } } → { workspace = true }\n"));
}

#[test]
fn installs_into_a_root_package() {
    let dir = project("root-package");

    assert!(install(&dir, &["-y"]).status.success());

    let cargo_toml = read(&dir, "Cargo.toml");
    assert!(cargo_toml.starts_with(&fixture("root-package/Cargo.toml")));
    assert!(cargo_toml.contains("\n[workspace.lints.clippy]\n"));
    assert!(cargo_toml.ends_with(INHERITED));
    assert!(clippy_is_silent(&dir));
}

#[test]
fn merges_into_a_hidden_clippy_toml() {
    let dir = project("crate");
    fs::rename(
        dir.path().join("clippy.toml"),
        dir.path().join(".clippy.toml"),
    )
    .unwrap();

    assert!(install(&dir, &["-y"]).status.success());

    assert!(read(&dir, ".clippy.toml").contains("cognitive-complexity-threshold = 12"));
    assert!(!dir.path().join("clippy.toml").exists());
}

#[test]
fn adds_missing_lints_back_to_their_blocks() {
    let dir = project("crate");
    assert!(install(&dir, &["-y"]).status.success());
    let installed = read(&dir, "Cargo.toml");
    let without = installed
        .replace("ffi_unwind_calls = \"warn\"\n", "")
        .replace("let_underscore_drop = \"warn\"\n", "");
    fs::write(dir.path().join("Cargo.toml"), without).unwrap();

    assert!(install(&dir, &["-y"]).status.success());

    assert_eq!(read(&dir, "Cargo.toml"), installed);
}

#[test]
fn second_run_changes_nothing() {
    let dir = project("workspace");
    assert!(install(&dir, &["-y"]).status.success());

    let output = install(&dir, &["-y"]);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.matches("Unchanged").count(), 4);
    assert!(!stdout.contains("Updated"));
}
