use std::process::{Command, Output};

use tempfile::TempDir;

use super::{BIN, fixture, project, read};

fn run_on(rust: &str, command: &mut Command, dir: &TempDir) -> Output {
    command
        .current_dir(dir.path())
        .env("RUSTUP_TOOLCHAIN", rust)
        .output()
        .unwrap()
}

fn install_and_lint(rust: &str) -> (TempDir, String) {
    let dir = project("crate");

    let install = run_on(rust, Command::new(BIN).arg("easy-peasy"), &dir);
    let warnings = String::from_utf8(install.stderr).unwrap();
    assert!(install.status.success(), "{warnings}");

    let clippy = run_on(
        rust,
        Command::new("cargo").args(["clippy", "--quiet"]),
        &dir,
    );
    let diagnostics = String::from_utf8(clippy.stderr).unwrap();
    assert!(
        clippy.status.success() && diagnostics.is_empty(),
        "{diagnostics}"
    );
    (dir, warnings)
}

#[test]
#[ignore = "needs Rust 1.50 with clippy, runs in CI"]
fn installs_only_settings_before_cargo_reads_lints() {
    let (dir, warnings) = install_and_lint("1.50");

    assert!(!warnings.contains("clippy::"), "{warnings}");
    assert_eq!(read(&dir, "Cargo.toml"), fixture("crate/Cargo.toml"));
    assert!(read(&dir, "clippy.toml").contains("cognitive-complexity-threshold"));
}

#[test]
#[ignore = "needs Rust 1.74 with clippy, runs in CI"]
fn skips_what_the_first_rust_with_lints_does_not_know() {
    let (dir, warnings) = install_and_lint("1.74");

    assert!(warnings.contains("allow-indexing-slicing-in-tests"));
    assert!(read(&dir, "Cargo.toml").contains("pedantic"));
    assert!(!read(&dir, "clippy.toml").contains("allow-indexing-slicing-in-tests"));
}
