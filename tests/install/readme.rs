use std::fs;

use tempfile::TempDir;

use super::{clippy_is_silent, install, project};

const README: &str = include_str!("../../README.md");

const READS: &str = "
fn read_primary() -> Vec<u8> {
    vec![0, 1, 0x81, 0x83]
}

fn read_replica() -> Vec<u8> {
    vec![0, 1]
}
";

/// Rust blocks of the README example: the original, then the fixed code.
fn example() -> Vec<&'static str> {
    README
        .split("\n## ")
        .find_map(|section| section.strip_prefix("Example\n"))
        .unwrap()
        .split("```rust\n")
        .skip(1)
        .map(|block| block.split_once("```").unwrap().0)
        .collect()
}

fn crate_with(code: &str) -> TempDir {
    let dir = project("root-package");
    fs::write(
        dir.path().join("src/lib.rs"),
        format!("//! Example.\n\n{code}{READS}"),
    )
    .unwrap();
    dir
}

#[test]
fn readme_example_has_two_blocks() {
    assert_eq!(example().len(), 2);
}

#[test]
fn readme_original_passes_default_lints() {
    assert!(clippy_is_silent(&crate_with(example()[0])));
}

#[test]
fn readme_original_fails_the_preset() {
    let dir = crate_with(example()[0]);
    assert!(install(&dir, &[]).status.success());
    assert!(!clippy_is_silent(&dir));
}

#[test]
fn readme_fixed_code_passes_the_preset() {
    let dir = crate_with(example()[1]);
    assert!(install(&dir, &[]).status.success());
    assert!(clippy_is_silent(&dir));
}
