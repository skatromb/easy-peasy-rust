//! Runs `cargo easy-peasy` on copies of `tests/fixtures` and checks what it writes.
#![cfg(test)]

mod commands;
mod layouts;

use std::fs;
use std::path::Path;
use std::process::{Command, Output, Stdio};

use rexpect::session::{PtySession, spawn_command};
use tempfile::TempDir;

const BIN: &str = env!("CARGO_BIN_EXE_cargo-easy-peasy");
const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

fn project(fixture: &str) -> TempDir {
    let dir = tempfile::tempdir().unwrap();
    copy(&Path::new(FIXTURES).join(fixture), dir.path());
    dir
}

fn copy(source: &Path, destination: &Path) {
    for entry in fs::read_dir(source).unwrap().map(Result::unwrap) {
        let target = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            fs::create_dir_all(&target).unwrap();
            copy(&entry.path(), &target);
        } else {
            let _ = fs::copy(entry.path(), target).unwrap();
        }
    }
}

fn fixture(name: &str) -> String {
    fs::read_to_string(Path::new(FIXTURES).join(name)).unwrap()
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

fn ask(dir: &TempDir, flags: &[&str]) -> PtySession {
    let mut command = Command::new(BIN);
    let _ = command.arg("easy-peasy").arg(dir.path()).args(flags);
    spawn_command(command, Some(30_000)).unwrap()
}

fn clippy_is_silent(dir: &TempDir) -> bool {
    let output = Command::new(env!("CARGO"))
        .args(["clippy", "--quiet", "--workspace", "--all-targets"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    output.status.success() && output.stderr.is_empty()
}
