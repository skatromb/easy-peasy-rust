# easy-peasy-rust

An opinionated lint preset for Rust, attempting to resemble the [wemake-python-styleguide](https://wemake-python-styleguide.readthedocs.io/).

## Usage

Run it inside a workspace or a single crate, or pass the path.

```sh
cargo install easy-peasy-rust
cargo easy-peasy [path]                 # asks about each of your settings that differ
cargo easy-peasy --diff                 # print the diff from `easy-peasy`, don't write anything

# If you're applying to an existing repo with an existing config:
cargo easy-peasy --yes                  # apply without asking
cargo easy-peasy --drop-existing        # drop your existing lints that `easy-peasy` does not list
cargo easy-peasy --yes --drop-existing  # all-in replace your settings with `easy-peasy` presets
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

Lints of other tools, like `[lints.rustdoc]`, and `clippy.toml` settings the preset does not set are never touched.

## Overriding

Per crate, at the top of `src/lib.rs` or `src/main.rs`:

```rust
#![allow(clippy::some_lint)]
```

Cargo does not allow a member to combine `workspace = true` with its own lint keys.

Per workspace: set the lint to `"allow"` in `Cargo.toml`, or change a threshold in `clippy.toml`. The next `cargo easy-peasy` asks about it again: answer `n` to keep yours.

## Versioning

`1.99.x` has the lints of Rust 1.99. Older toolchains get only the lints and settings they know, per [`validity.toml`](rules/validity.toml): upgrade Rust and rerun `cargo easy-peasy` to add the rest.

## License

MIT
