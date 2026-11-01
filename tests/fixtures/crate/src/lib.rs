//! Fixture.

use std::str::Chars;

/// Takes `'_`, which `min_ident_chars` flags before Rust 1.99.
#[must_use]
pub fn count(chars: Chars<'_>) -> usize {
    chars.count()
}
