# easy-peasy-rust

[![Check](https://github.com/skatromb/easy-peasy-rust/actions/workflows/check.yml/badge.svg)](https://github.com/skatromb/easy-peasy-rust/actions/workflows/check.yml)
[![Crates.io](https://img.shields.io/crates/v/easy-peasy-rust.svg)](https://crates.io/crates/easy-peasy-rust)
[![Downloads](https://img.shields.io/crates/dr/easy-peasy-rust.svg?label=downloads%2F90d)](https://crates.io/crates/easy-peasy-rust)
[![MSRV](https://img.shields.io/crates/msrv/easy-peasy-rust.svg)](https://crates.io/crates/easy-peasy-rust)
[![easy-peasy-rust](https://img.shields.io/badge/style-easy--peasy-fecc02.svg)](https://github.com/skatromb/easy-peasy-rust)

An opinionated lint preset for Rust, attempting to resemble the [wemake-python-styleguide](https://wemake-python-styleguide.readthedocs.io/).

## Usage

Run it inside a workspace or a single crate, or pass the path.

```sh
cargo install easy-peasy-rust
cargo easy-peasy [path]           # add what you lack, keep your lints
cargo easy-peasy --drop-existing  # apply the whole preset, drop your other lints
cargo easy-peasy --interactive    # ask about each block of lint rules that differs
cargo easy-peasy --diff           # only print the diff and fail on some
```

Then use your IDE as usual or run clippy manually:

```sh
cargo clippy --workspace --all-targets
```

## Rules

Every lint pick is explained in [`lints.toml`](rules/lints.toml) and [`clippy.toml`](rules/clippy.toml).

## What it writes

- `[workspace.lints.rust]` and `[workspace.lints.clippy]` into the root `Cargo.toml` (`[lints.*]` for a single crate),
- `[lints] workspace = true` into every member crate, the root package included,
- the settings into `clippy.toml`, or into `.clippy.toml` if that is what you have.

Each preset block gets a `# easy-peasy: <block>` header, in the preset's order, and your own lints and settings go below them under `# easy-peasy: yours`.

Lints of other tools, like `[lints.rustdoc]`, are never touched.

## Overriding

Per crate, at the top of `src/lib.rs` or `src/main.rs`:

```rust
#![allow(clippy::some_lint)]
```

Cargo does not allow a member to combine `workspace = true` with its own lint keys.

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
