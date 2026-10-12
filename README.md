# easy-peasy-rust

[![Check](https://github.com/skatromb/easy-peasy-rust/actions/workflows/check.yml/badge.svg)](https://github.com/skatromb/easy-peasy-rust/actions/workflows/check.yml)
[![Rust](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Fskatromb%2Feasy-peasy-rust%2Fmain%2FCargo.toml&query=%24.package.rust-version&prefix=%E2%89%A5&label=rust)](https://crates.io/crates/easy-peasy-rust)
[![Crates.io](https://img.shields.io/crates/v/easy-peasy-rust.svg)](https://crates.io/crates/easy-peasy-rust)
[![Downloads](https://img.shields.io/crates/d/easy-peasy-rust.svg)](https://crates.io/crates/easy-peasy-rust)
[![easy-peasy-rust](https://img.shields.io/badge/style-easy--peasy-fecc02.svg)](https://github.com/skatromb/easy-peasy-rust)

An opinionated lint preset for Rust. Keeps human- and AI-written code readable.

Inspired by [wemake-python-styleguide](https://wemake-python-styleguide.readthedocs.io/).

## Usage

Run it inside a workspace or a single crate, or pass the path.

```sh
cargo install easy-peasy-rust
cargo easy-peasy [path]           # add what you lack, keep your lints
cargo easy-peasy --drop-existing  # apply the whole preset, drop your other lints
cargo easy-peasy --interactive    # ask about each block of lint rules that differs
cargo easy-peasy --diff           # only print the diff and fail on some
```

Then let your IDE run clippy, or run it yourself:

```sh
cargo clippy --workspace --all-targets
```

## Rules

Every lint pick is explained in [`lints.toml`](rules/lints.toml) and [`clippy.toml`](rules/clippy.toml).

## What it writes

- `Cargo.toml`: the preset lints, and `[lints] workspace = true` in every member crate
- `clippy.toml`: lint settings

## Overriding

Per crate, at the top of `src/lib.rs` or `src/main.rs`:

```rust
#![allow(clippy::some_lint)]
```

Cargo forbids own lint keys next to `workspace = true`.

Per workspace: set the lint to `"allow"` in `Cargo.toml`, or change a threshold in `clippy.toml`. The next `cargo easy-peasy` keeps it, `--drop-existing` resets it.

## Versioning

`1.99.x` has the lints of Rust 1.99. Older toolchains get only the lints and settings they know, per [`validity.toml`](rules/validity.toml): upgrade Rust and rerun `cargo easy-peasy` to add the rest.

## Badge

[![easy-peasy-rust](https://img.shields.io/badge/style-easy--peasy-fecc02.svg)](https://github.com/skatromb/easy-peasy-rust)

```md
[![easy-peasy-rust](https://img.shields.io/badge/style-easy--peasy-fecc02.svg)](https://github.com/skatromb/easy-peasy-rust)
```

## License

MIT
