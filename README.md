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

Run it anywhere inside a workspace or a single crate, or pass the path. It writes:

- `[workspace.lints.clippy]` and `[workspace.lints.rust]` into the root `Cargo.toml`
  (`[lints.*]` for a single crate),
- `[lints] workspace = true` into every member crate, the root package included,
- the thresholds into `clippy.toml`, or into `.clippy.toml` if that is what you have.

For each setting you already have that differs from the preset, it shows the lint's docs and asks:

```
clippy::unwrap_used: yours "allow", preset "deny"
  https://rust-lang.github.io/rust-clippy/master/index.html#unwrap_used
Take the preset's value? [y/N]
```

Enter keeps yours. Pass `-y` (`--overwrite`) to take the preset's value on every conflict; without a
terminal to ask in, it is required. A member crate with its own `[lints]` is a conflict too: keeping
yours leaves that crate out of the preset. Re-run after upgrading to pick up new lints.

The preset decides every lint: what it does not list stays at its default, or at its group's level.
So a lint you set that the preset does not, like `elided_lifetimes_in_paths = "warn"` or
`clippy::shadow_reuse = "allow"`, moves you away from it. These are kept, and listed in a warning.
Pass `--drop-existing` to treat them as conflicts too, where taking the preset's value drops yours:

```sh
cargo easy-peasy --drop-existing [--overwrite]
```

Lints of other tools, like `[lints.rustdoc]`, and `clippy.toml` settings the preset does not set
are never touched.

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
Rust 1.99. A workspace pinned to an older toolchain still works, but that rustc prints an
`unknown lint` warning for every lint it does not have yet, so the installer warns you up front.

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
| WPS425 positional bools         | forbidden             | `max-fn-params-bools`                     | 0, no bool parameters      |

Of the 198 active WPS rules, 43 have a clippy or rustc counterpart that is on, 32 one that covers
them in part. 79 do not apply to Rust. 44 apply but have no lint yet: magic numbers, counts of
locals, returns, methods and module items, overused expressions, `await` in a loop. Those would
need a [dylint](https://github.com/trailofbits/dylint) library. The
[WPS mapping](rules/external/wps_mapping.md) has the verdict for every rule. The reasoning for
every lint and setting sits next to it in [`lints.toml`](rules/lints.toml) and
[`clippy.toml`](rules/clippy.toml).

## License

MIT
