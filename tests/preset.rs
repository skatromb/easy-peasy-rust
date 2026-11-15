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
const RUSTDOC: &str = include_str!("../rules/external/rustdoc/allowed-by-default.md");
const CARGO_PEDANTIC: &str = include_str!("../rules/external/cargo/pedantic.md");
const CARGO_RESTRICTION: &str = include_str!("../rules/external/cargo/restriction.md");
const LISTINGS: [&str; 9] = [
    CARGO,
    NURSERY,
    PEDANTIC,
    RESTRICTION,
    SETTINGS,
    RUSTC,
    RUSTDOC,
    CARGO_PEDANTIC,
    CARGO_RESTRICTION,
];
const TOOLCHAIN: &str = include_str!("../rust-toolchain.toml");
const CLIPPY_GROUPS: [&str; 9] = [
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
const CARGO_GROUPS: [&str; 5] = [
    "correctness",
    "pedantic",
    "restriction",
    "style",
    "suspicious",
];

#[test]
fn validity_matches_preset() {
    let mut lints: DocumentMut = PRESETS[0].parse().unwrap();
    let settings: DocumentMut = PRESETS[1].parse().unwrap();
    let _ = lints.insert("clippy.toml", Item::Table(settings.as_table().clone()));

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
fn listings_match_toolchain() {
    let toolchain: DocumentMut = TOOLCHAIN.parse().unwrap();
    let rust = toolchain["toolchain"]["channel"].as_str().unwrap();
    for listing in LISTINGS {
        let (_, header) = listing.split_once("as of Rust ").unwrap();
        let scraped = header
            .split_whitespace()
            .next()
            .unwrap()
            .trim_end_matches([',', '.']);
        assert_eq!(scraped, rust, "run `make scrape` for Rust {rust}");
    }
}

#[test]
fn every_clippy_lint_decided() {
    let required = listed(RESTRICTION).into_iter().chain(CLIPPY_GROUPS);
    let listings = [CARGO, NURSERY, PEDANTIC, RESTRICTION];
    let known = listings.into_iter().flat_map(listed).chain(CLIPPY_GROUPS);
    assert_decided(sections("clippy"), &required.collect(), &known.collect());
}

#[test]
fn every_rustc_lint_decided() {
    assert_decided(sections("rust"), &listed(RUSTC), &listed(RUSTC));
}

#[test]
fn every_rustdoc_lint_decided() {
    assert_decided(sections("rustdoc"), &listed(RUSTDOC), &listed(RUSTDOC));
}

#[test]
fn every_cargo_lint_decided() {
    let required = listed(CARGO_RESTRICTION).into_iter().chain(CARGO_GROUPS);
    let listings = [CARGO_PEDANTIC, CARGO_RESTRICTION];
    let known = listings.into_iter().flat_map(listed).chain(CARGO_GROUPS);
    assert_decided(sections("cargo"), &required.collect(), &known.collect());
}

fn sections(tool: &str) -> [&'static str; 2] {
    let header = format!("[{tool}]\n");
    [PRESETS[0], DEFAULTS[0]].map(|lints| {
        let (_, rest) = lints.split_once(&header).unwrap();
        rest.split("\n[").next().unwrap()
    })
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
fn preset_keys_sorted_groups_first() {
    for block in PRESETS
        .iter()
        .chain(&DEFAULTS)
        .flat_map(|file| file.split("\n\n"))
    {
        let keys: Vec<_> = block
            .lines()
            .filter_map(|line| Some((!line.contains("priority = -1"), name(line)?)))
            .collect();
        assert!(keys.is_sorted(), "not sorted: {keys:?}");
    }
}

fn name(line: &str) -> Option<&str> {
    let (key, _) = line.split_once(" = ")?;
    (!key.starts_with('#')).then_some(key)
}
