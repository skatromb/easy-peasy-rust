# easy-peasy-rust

An opinionated lint preset for Rust, in the spirit of
[wemake-python-styleguide](https://wemake-python-styleguide.readthedocs.io/): every clippy and
rustc lint on, the ones that fight readable code allowed, thresholds tuned for small functions and
descriptive names. One command installs it into your workspace.

## Install

```sh
cargo install easy-peasy-rust
cargo easy-peasy
```

Run it at the root of a workspace or a single crate. It writes:

- `[workspace.lints.clippy]` and `[workspace.lints.rust]` into the root `Cargo.toml`
  (`[lints.*]` for a single crate),
- `[lints] workspace = true` into every member crate,
- `clippy.toml` with the thresholds.

For each setting you already have that differs from the preset, it shows the lint's docs and asks:

```
clippy::unwrap_used: yours "allow", preset "deny"
  https://rust-lang.github.io/rust-clippy/master/index.html#unwrap_used
Take the preset's value? [y/N]
```

Enter keeps yours, and so does a non-interactive run. Pass `-y` (`--override`) to take the preset's
value on every conflict. Keys the preset does not know about are never touched. Re-run after
upgrading to pick up new lints.

Then:

```sh
cargo clippy --workspace --all-targets
```

## Overriding

Per crate: `#![allow(clippy::some_lint)]` in the crate root. Cargo does not allow a member to
combine `workspace = true` with its own lint keys.

Per workspace: edit the values in `Cargo.toml` and `clippy.toml`. The next `cargo easy-peasy` keeps
your edits unless you answer `y` or pass `-y`.

## Versioning

The crate version tracks the Rust release the preset was curated against: `1.99.x` targets
Rust 1.99. On an older toolchain unknown lint names only trigger `unknown_lints`, but that fails
`-D warnings`, so match the version to your toolchain: `cargo install easy-peasy-rust@1.95`.

## From WPS to clippy

| WPS                             | WPS default           | clippy                                    | preset                     |
| ------------------------------- | --------------------- | ----------------------------------------- | -------------------------- |
| WPS110 variable blacklist       | 37 names              | `disallowed-names`                        | adjusted to the Rust world |
| WPS111 short names              | `min-name-length = 2` | `min-ident-chars-threshold`               | 2, no allowlist            |
| WPS211 arguments                | 5                     | `too-many-arguments-threshold`            | 4                          |
| WPS213 expressions per function | 9                     | `too-many-lines-threshold`                | 25                         |
| WPS220 nesting                  | 5                     | `excessive-nesting-threshold`             | 4                          |
| WPS231 cognitive complexity     | 12                    | `cognitive-complexity-threshold`          | 12                         |
| WPS234 annotation complexity    | 3                     | `type-complexity-threshold`               | 75                         |
| WPS425 bool arguments           | —                     | `max-fn-params-bools`, `max-struct-bools` | 1                          |

Of the 264 WPS rules, 83 have a clippy or rustc counterpart and are on. 43 are impossible in Rust
by construction, 80 are Python-only. 50 apply in principle but have no lint yet: magic numbers,
counts of locals, returns, methods and module items, overused expressions, `await` in a loop. Those
would need a [dylint](https://github.com/trailofbits/dylint) library.

## License

MIT
