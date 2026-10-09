use rexpect::session::PtySession;

use super::{ask, clippy_is_silent, fixture, install, project, read};

#[test]
fn refuses_to_run_without_a_terminal() {
    let dir = project("crate");

    let output = install(&dir, &[]);

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr).unwrap().contains("--yes"));
    assert_eq!(read(&dir, "Cargo.toml"), fixture("crate/Cargo.toml"));
    assert_eq!(read(&dir, "clippy.toml"), fixture("crate/clippy.toml"));
}

#[test]
fn asks_about_each_block() {
    let dir = project("crate");

    let blocks = answer(ask(&dir, &[]), |block| {
        if block.contains("unwrap_used") {
            "n"
        } else {
            ""
        }
    });

    let panics = blocks.iter().find(|block| block.contains("No panics"));
    assert!(panics.unwrap().contains("clippy::unwrap_used: \"allow\""));
    let extras = blocks.last().unwrap();
    assert!(extras.contains("You have clippy lints that are not in `easy-peasy-rust`"));
    assert!(extras.contains("clippy::float_arithmetic"));
    let cargo_toml = read(&dir, "Cargo.toml");
    assert!(cargo_toml.contains("unwrap_used = \"allow\"\n"));
    assert!(!cargo_toml.contains("expect_used"));
    assert!(cargo_toml.contains("float_arithmetic = \"allow\"\n"));
    assert!(read(&dir, "clippy.toml").contains("too-many-lines-threshold = 20"));
}

#[test]
fn matches_the_preset_after_yes_to_everything() {
    let dir = project("crate");

    let blocks = answer(ask(&dir, &[]), |_| "y");

    assert!(!blocks.iter().any(|block| block.contains("rustdoc")));
    let cargo_toml = read(&dir, "Cargo.toml");
    assert!(!cargo_toml.contains("float_arithmetic"));
    assert!(cargo_toml.contains("broken_intra_doc_links"));
    let diff = install(&dir, &["--diff"]);
    assert_eq!(diff.stdout, b"Your settings match the preset\n");
}

#[test]
fn lists_differences_without_writing_with_diff() {
    let dir = project("crate");

    let output = install(&dir, &["--diff"]);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("  clippy::unwrap_used: \"allow\" → \"deny\"\n"));
    assert!(stdout.contains("  clippy::panic: \"deny\"\n"));
    assert!(stdout.contains("  clippy::float_arithmetic\n"));
    assert!(stdout.contains("  too-many-lines-threshold: 50 → 20\n"));
    assert!(!stdout.contains("[Y/n]"));
    assert_eq!(read(&dir, "Cargo.toml"), fixture("crate/Cargo.toml"));
    assert_eq!(read(&dir, "clippy.toml"), fixture("crate/clippy.toml"));
}

#[test]
fn takes_the_preset_and_keeps_your_other_lints_with_yes() {
    let dir = project("crate");

    let output = install(&dir, &["--yes"]);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("  clippy::unwrap_used: \"allow\" → \"deny\"\n"));
    assert!(stdout.contains("Adopt? [Y/n] y\n"));
    assert!(stdout.contains("  clippy::float_arithmetic\n"));
    assert!(stdout.contains("Remove? [y/N] n\n"));
    let cargo_toml = read(&dir, "Cargo.toml");
    assert!(cargo_toml.contains("unwrap_used = \"deny\"\n"));
    assert!(cargo_toml.contains("\n\n# No panics\narithmetic_side_effects = \"deny\"\n"));
    assert!(cargo_toml.contains("float_arithmetic = \"allow\"\n"));
    assert!(cargo_toml.contains("\n[lints.rust]\n"));
    let clippy_toml = read(&dir, "clippy.toml");
    assert!(clippy_toml.contains("too-many-lines-threshold = 20"));
    assert!(clippy_toml.contains("msrv = \"1.85\"\n"));
    assert!(clippy_is_silent(&dir));
}

fn answer(mut session: PtySession, reply: impl Fn(&str) -> &'static str) -> Vec<String> {
    let mut blocks = Vec::new();
    while let Ok((block, _)) = session.exp_regex(r"Adopt\? \[Y/n\] |Remove\? \[y/N\] ") {
        let _ = session.send_line(reply(&block)).unwrap();
        blocks.push(block);
    }
    blocks
}
