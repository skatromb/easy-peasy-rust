# easy-peasy-rust

- The crate dogfoods its own preset: `[lints.clippy]` and `[lints.rust]` in `Cargo.toml` are `[clippy]` and `[rust]` from `preset/lints.toml` verbatim, and `clippy.toml` links to `preset/clippy.toml`. Change the preset first, then mirror it in `Cargo.toml`.
- Run `make lint` and `make test` after every change; lint must pass with `-D warnings`.
- Tests return `()` and `unwrap`: `panic_in_result_fn` forbids `assert!` in a test that returns `Result`. Helpers outside `#[test]` functions return `Result`, since `unwrap` is allowed only inside tests.
- Never silence a lint with `#[allow]` or `#[expect]` without asking. If a lint hurts, it will hurt users too: propose a preset change instead.
- Print through `writeln!` on a locked `stdout`/`stderr`, since `print_stdout` and `print_stderr` are on.
