# easy-peasy-rust

- The crate dogfoods its own preset: `[lints.clippy]` and `[lints.rust]` in `Cargo.toml` are `[clippy]` and `[rust]` from `rules/lints.toml` verbatim, and `clippy.toml` links to `rules/clippy.toml`. Change the preset first, then mirror it in `Cargo.toml`.
- Run `make lint` and `make test` after every change; lint must pass with `-D warnings`.
- Integration test files start with `#![cfg(test)]`: `tests_outside_test_module` wants tests inside `cfg(test)`, and it lets helpers `unwrap` too. Tests return `()`, since `panic_in_result_fn` forbids `assert!` in a test that returns `Result`.
- Never silence a lint with `#[allow]` or `#[expect]` without asking. If a lint hurts, it will hurt users too: propose a preset change instead.
- Print through `writeln!` on a locked `stdout`/`stderr`, since `print_stdout` and `print_stderr` are on.
