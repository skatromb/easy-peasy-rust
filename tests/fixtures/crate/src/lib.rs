//! Fixture that tests version-specific behaviours.

use std::fs;
use std::str::Chars;

/// `'_` is flagged by `min_ident_chars` before Rust 1.99.
#[must_use]
pub fn count(chars: Chars<'_>) -> usize {
    chars.count()
}

/// `fs` is flagged by `min_ident_chars` even in `use` before Rust 1.99.
#[must_use]
pub fn exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
