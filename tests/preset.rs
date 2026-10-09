//! Checks the preset files themselves.
#![cfg(test)]

use std::collections::BTreeSet;

use toml_edit::{DocumentMut, Item};

const PRESETS: [&str; 2] = [
    include_str!("../rules/lints.toml"),
    include_str!("../rules/clippy.toml"),
];
const VALIDITY: &str = include_str!("../rules/validity.toml");
const RESTRICTION: &str = include_str!("../rules/external/clippy/restriction.md");

#[test]
fn validity_lists_exactly_the_preset_entries() {
    let mut lints: DocumentMut = PRESETS[0].parse().unwrap();
    let settings: DocumentMut = PRESETS[1].parse().unwrap();
    drop(lints.insert("clippy.toml", Item::Table(settings.as_table().clone())));

    let preset = entries(&lints);
    let validity = entries(&VALIDITY.parse().unwrap());

    let missing: Vec<&String> = preset.difference(&validity).collect();
    let stale: Vec<&String> = validity.difference(&preset).collect();
    assert!(
        missing.is_empty() && stale.is_empty(),
        "missing: {missing:?}, stale: {stale:?}"
    );
}

fn entries(document: &DocumentMut) -> BTreeSet<String> {
    document
        .iter()
        .flat_map(|(section, table)| {
            let names = table.as_table().unwrap().iter();
            names.map(move |(name, _)| format!("{section}.{name}"))
        })
        .collect()
}

#[test]
fn decides_every_restriction_lint() {
    let (_, clippy) = PRESETS[0].split_once("[clippy]").unwrap();
    let decided: BTreeSet<&str> = clippy.lines().filter_map(name).collect();
    let left_out: BTreeSet<&str> = clippy
        .lines()
        .filter(|line| line.starts_with("# - "))
        .filter_map(name)
        .collect();
    let restriction: BTreeSet<&str> = RESTRICTION
        .lines()
        .filter_map(|line| line.strip_prefix("| [`")?.split('`').next())
        .collect();

    let undecided: Vec<&&str> = restriction.difference(&decided).collect();
    let stale: Vec<&&str> = left_out.difference(&restriction).collect();
    assert!(
        undecided.is_empty() && stale.is_empty(),
        "undecided: {undecided:?}, stale: {stale:?}"
    );
}

#[test]
fn sorts_keys_within_each_block() {
    for block in PRESETS.iter().flat_map(|preset| preset.split("\n\n")) {
        let names: Vec<&str> = block.lines().filter_map(name).collect();
        assert!(names.is_sorted(), "not sorted: {names:?}");
    }
}

fn name(line: &str) -> Option<&str> {
    match line.strip_prefix("# - ") {
        Some(bullet) => bullet.split([':', ' ']).next(),
        None if line.starts_with('#') => None,
        None => line.split_once(" = ").map(|(key, _)| key),
    }
}
