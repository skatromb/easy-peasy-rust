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

    let questions = answer(ask(&dir, &[]), "");

    let panics = questions.iter().find(|asked| asked.contains("No panics"));
    assert!(panics.unwrap().contains("clippy::expect_used: \"deny\""));
    let yours = questions
        .iter()
        .find(|asked| asked.contains("clippy::unwrap_used"));
    assert!(yours.unwrap().contains("Replace yours? [y/N]"));
    let extras = questions.last().unwrap();
    assert!(extras.contains("You have clippy lints that are not in `easy-peasy-rust`"));
    assert!(extras.contains("clippy::float_arithmetic"));
    let cargo_toml = read(&dir, "Cargo.toml");
    assert!(cargo_toml.contains("expect_used = \"deny\"\n"));
    assert!(cargo_toml.contains("unwrap_used = \"allow\"\n"));
    assert!(cargo_toml.contains("float_arithmetic = \"allow\"\n"));
    let clippy_toml = read(&dir, "clippy.toml");
    assert!(clippy_toml.contains("cognitive-complexity-threshold = 12"));
    assert!(clippy_toml.contains("too-many-lines-threshold = 50"));
}

#[test]
fn matches_the_preset_after_yes_to_everything() {
    let dir = project("crate");

    let questions = answer(ask(&dir, &[]), "y");

    assert!(!questions.iter().any(|asked| asked.contains("rustdoc")));
    let cargo_toml = read(&dir, "Cargo.toml");
    assert!(!cargo_toml.contains("float_arithmetic"));
    assert!(cargo_toml.contains("broken_intra_doc_links"));
    let diff = install(&dir, &["--diff"]);
    assert_eq!(diff.stdout, b"Your settings match the preset\n");
}

#[test]
fn replaces_a_workspace_members_own_lints_on_yes() {
    let dir = project("workspace");

    let questions = answer(ask(&dir, &[]), "y");

    let crates = questions.iter().find(|asked| asked.contains("crates/two"));
    assert!(crates.unwrap().contains("Replace yours? [y/N]"));
    let member_toml = read(&dir, "crates/two/Cargo.toml");
    assert!(member_toml.ends_with("\n[lints]\nworkspace = true\n"));
    assert!(!member_toml.contains("unwrap_used"));
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
fn adds_the_preset_and_keeps_yours_with_yes() {
    let dir = project("crate");

    let output = install(&dir, &["--yes"]);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Adopt? [Y/n] y\n"));
    assert!(
        stdout.contains("  clippy::unwrap_used: \"allow\" → \"deny\"\nReplace yours? [y/N] n\n")
    );
    assert!(stdout.contains("  clippy::float_arithmetic\nRemove? [y/N] n\n"));
    let cargo_toml = read(&dir, "Cargo.toml");
    assert!(cargo_toml.contains("unwrap_used = \"allow\"\n"));
    assert!(cargo_toml.contains("\n\n# No panics\narithmetic_side_effects = \"deny\"\n"));
    assert!(cargo_toml.contains("float_arithmetic = \"allow\"\n"));
    assert!(cargo_toml.contains("\n[lints.rust]\n"));
    let clippy_toml = read(&dir, "clippy.toml");
    assert!(clippy_toml.contains("too-many-lines-threshold = 50"));
    assert!(clippy_toml.contains("msrv = \"1.85\"\n"));
    assert!(clippy_is_silent(&dir));
}

#[test]
fn applies_the_whole_preset_with_drop_existing() {
    let dir = project("crate");

    assert!(install(&dir, &["--drop-existing"]).status.success());

    assert!(!read(&dir, "Cargo.toml").contains("float_arithmetic"));
    let diff = install(&dir, &["--diff"]);
    assert_eq!(diff.stdout, b"Your settings match the preset\n");
}

fn answer(mut session: PtySession, reply: &str) -> Vec<String> {
    let mut questions = Vec::new();
    while let Ok((lines, prompt)) = session.exp_regex(r"\? \[(Y/n|y/N)\] ") {
        let asked = format!("{lines}{prompt}");
        let _ = session.send_line(reply).unwrap();
        questions.push(asked);
    }
    questions
}
