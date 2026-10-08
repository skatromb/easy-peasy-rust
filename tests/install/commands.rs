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
fn asks_about_each_conflict() {
    let dir = project("crate");

    let mut session = ask(&dir, &[]);
    let first = session.exp_string("[Y/n] ").unwrap();
    let _ = session.send_line("n").unwrap();
    let second = session.exp_string("[Y/n] ").unwrap();
    let _ = session.send_line("").unwrap();
    let rest = session.exp_eof().unwrap();

    assert!(first.contains("clippy::unwrap_used"));
    assert!(second.contains("too-many-lines-threshold"));
    assert!(rest.contains("Updated clippy.toml"));
    assert!(read(&dir, "Cargo.toml").contains("unwrap_used = \"allow\"\n"));
    assert!(read(&dir, "clippy.toml").contains("too-many-lines-threshold = 20"));
}

#[test]
fn lists_differences_without_writing_with_diff() {
    let dir = project("crate");

    let output = install(&dir, &["--diff"]);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("clippy::unwrap_used: yours \"allow\", preset \"deny\"\n"));
    assert!(stdout.contains("clippy::dbg_macro: yours \"allow\", preset not set\n"));
    assert!(stdout.contains("clippy::panic: yours not set, preset \"deny\"\n"));
    assert!(stdout.contains("too-many-lines-threshold: yours 50, preset 20\n"));
    assert_eq!(read(&dir, "Cargo.toml"), fixture("crate/Cargo.toml"));
    assert_eq!(read(&dir, "clippy.toml"), fixture("crate/clippy.toml"));
}

#[test]
fn takes_the_preset_and_keeps_your_other_lints_with_yes() {
    let dir = project("crate");

    let output = install(&dir, &["--yes"]);

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stdout.contains("clippy::unwrap_used: yours \"allow\", preset \"deny\"\n"));
    assert!(stdout.contains("Replaced yours\n"));
    assert!(stderr.contains("clippy::dbg_macro: yours \"allow\", preset not set\n"));
    assert!(stderr.contains("--drop-existing"));
    let cargo_toml = read(&dir, "Cargo.toml");
    assert!(cargo_toml.contains("unwrap_used = \"deny\" # Panics.\n"));
    assert!(cargo_toml.contains("dbg_macro = \"allow\"\n"));
    assert!(cargo_toml.contains("\n[lints.rust]\n"));
    let clippy_toml = read(&dir, "clippy.toml");
    assert!(clippy_toml.contains("too-many-lines-threshold = 20"));
    assert!(clippy_toml.contains("msrv = \"1.85\"\n"));
    assert!(clippy_is_silent(&dir));
}

#[test]
fn asks_before_dropping_lints_the_preset_does_not_set() {
    let dir = project("crate");

    let mut session = ask(&dir, &["--drop-existing"]);
    for _ in 0..3 {
        let question = session.exp_string("[Y/n] ").unwrap();
        assert!(!question.contains("rustdoc"));
        let _ = session.send_line("").unwrap();
    }
    let rest = session.exp_eof().unwrap();

    assert!(rest.contains("Updated Cargo.toml"));
    let cargo_toml = read(&dir, "Cargo.toml");
    assert!(!cargo_toml.contains("dbg_macro"));
    assert!(cargo_toml.contains("broken_intra_doc_links"));
}

#[test]
fn matches_the_preset_with_yes_and_drop_existing() {
    let dir = project("crate");

    let output = install(&dir, &["--yes", "--drop-existing"]);

    assert_eq!(output.stderr, b"");
    assert!(read(&dir, "Cargo.toml").contains("broken_intra_doc_links"));
    let diff = install(&dir, &["--diff"]);
    assert_eq!(diff.stdout, b"Your settings match the preset\n");
}
