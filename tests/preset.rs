//! Checks the preset files themselves.
#![cfg(test)]

const PRESETS: [&str; 2] = [
    include_str!("../rules/lints.toml"),
    include_str!("../rules/clippy.toml"),
];

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
