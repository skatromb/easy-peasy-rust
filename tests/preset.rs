//! Checks the preset files themselves.
#![cfg(test)]

use std::collections::BTreeSet;

use toml_edit::{DocumentMut, Item};

const PRESETS: [&str; 2] = [
    include_str!("../rules/lints.toml"),
    include_str!("../rules/clippy.toml"),
];
const DEFAULTS: [&str; 2] = [
    include_str!("../rules/lints.defaults.toml"),
    include_str!("../rules/clippy.defaults.toml"),
];
const VALIDITY: &str = include_str!("../rules/validity.toml");
const CARGO: &str = include_str!("../rules/external/clippy/cargo.md");
const NURSERY: &str = include_str!("../rules/external/clippy/nursery.md");
const PEDANTIC: &str = include_str!("../rules/external/clippy/pedantic.md");
const RESTRICTION: &str = include_str!("../rules/external/clippy/restriction.md");
const SETTINGS: &str = include_str!("../rules/external/clippy/settings.md");
const RUSTC: &str = include_str!("../rules/external/rustc/allowed-by-default.md");
const GROUPS: [&str; 9] = [
    "cargo",
    "complexity",
    "correctness",
    "nursery",
    "pedantic",
    "perf",
    "restriction",
    "style",
    "suspicious",
];

#[test]
fn validity_matches_preset() {
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
fn every_clippy_lint_decided() {
    let files = [PRESETS[0], DEFAULTS[0]].map(|lints| lints.split_once("[clippy]").unwrap().1);
    let suspicious = ["cast_abs_to_unsigned"];
    let required = listed(RESTRICTION).into_iter().chain(GROUPS);
    let listings = [CARGO, NURSERY, PEDANTIC, RESTRICTION];
    let known = listings
        .into_iter()
        .flat_map(listed)
        .chain(GROUPS)
        .chain(suspicious);
    assert_decided(files, &required.collect(), &known.collect());
}

#[test]
fn every_rustc_lint_decided() {
    let files = [PRESETS[0], DEFAULTS[0]].map(|lints| lints.split_once("[clippy]").unwrap().0);
    assert_decided(files, &listed(RUSTC), &listed(RUSTC));
}

#[test]
fn every_clippy_setting_decided() {
    assert_decided(
        [PRESETS[1], DEFAULTS[1]],
        &listed(SETTINGS),
        &listed(SETTINGS),
    );
}

fn assert_decided(files: [&str; 2], required: &BTreeSet<&str>, known: &BTreeSet<&str>) {
    let [preset, defaults] =
        files.map(|file| file.lines().filter_map(name).collect::<BTreeSet<_>>());
    let decided = &preset | &defaults;
    let undecided: Vec<&&str> = required.difference(&decided).collect();
    let unknown: Vec<&&str> = decided.difference(known).collect();
    let twice: Vec<&&str> = preset.intersection(&defaults).collect();
    assert!(
        undecided.is_empty() && unknown.is_empty() && twice.is_empty(),
        "undecided: {undecided:?}, unknown: {unknown:?}, twice: {twice:?}"
    );
}

fn listed(listing: &str) -> BTreeSet<&str> {
    listing
        .lines()
        .filter_map(|line| line.strip_prefix("| [`")?.split('`').next())
        .collect()
}

#[test]
fn preset_keys_sorted() {
    for block in PRESETS
        .iter()
        .chain(&DEFAULTS)
        .flat_map(|file| file.split("\n\n"))
    {
        let names: Vec<&str> = block.lines().filter_map(name).collect();
        assert!(names.is_sorted(), "not sorted: {names:?}");
    }
}

fn name(line: &str) -> Option<&str> {
    let (key, _) = line.split_once(" = ")?;
    (!key.starts_with('#')).then_some(key)
}
