# rustc allowed by default

Every stable rustc lint that is allowed by default as of Rust 1.99, besides edition migrations that `cargo fix --edition` turns on.

| Lint | Description |
| --- | --- |
| [`ambiguous_negative_literals`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#ambiguous-negative-literals) | ambiguous negative literals operations |
| [`closure_returning_async_block`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#closure-returning-async-block) | closure that returns `async {}` could be rewritten as an async closure |
| [`dead_code_pub_in_binary`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#dead-code-pub-in-binary) | detect public items in executable crates that are never used |
| [`deprecated_in_future`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#deprecated-in-future) | detects use of items that will be deprecated in a future version |
| [`deref_into_dyn_supertrait`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#deref-into-dyn-supertrait) | `Deref` implementation with a supertrait trait object for output is shadowed by trait upcasting |
| [`elided_lifetimes_in_paths`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#elided-lifetimes-in-paths) | hidden lifetime parameters in types are deprecated |
| [`explicit_outlives_requirements`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#explicit-outlives-requirements) | outlives requirements can be inferred |
| [`ffi_unwind_calls`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#ffi-unwind-calls) | call to foreign functions or function pointers with FFI-unwind ABI |
| [`impl_trait_redundant_captures`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#impl-trait-redundant-captures) | redundant precise-capturing `use<...>` syntax on an `impl Trait` |
| [`let_underscore_drop`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#let-underscore-drop) | non-binding let on a type that has a destructor |
| [`linker_info`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#linker-info) | linker warnings known to be informational-only and not indicative of a problem |
| [`macro_use_extern_crate`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#macro-use-extern-crate) | the `#[macro_use]` attribute is now deprecated in favor of using macros via the module system |
| [`meta_variable_misuse`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#meta-variable-misuse) | possible meta-variable misuse at macro definition |
| [`missing_copy_implementations`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#missing-copy-implementations) | detects potentially-forgotten implementations of `Copy` |
| [`missing_debug_implementations`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#missing-debug-implementations) | detects missing implementations of Debug |
| [`missing_docs`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#missing-docs) | detects missing documentation for public members |
| [`non_ascii_idents`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#non-ascii-idents) | detects non-ASCII identifiers |
| [`raw_borrows_via_references`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#raw-borrows-via-references) | creating raw borrows via references is discouraged |
| [`redundant_imports`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#redundant-imports) | imports that are redundant due to being imported already |
| [`redundant_lifetimes`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#redundant-lifetimes) | detects lifetime parameters that are redundant because they are equal to some other named lifetime |
| [`single_use_lifetimes`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#single-use-lifetimes) | detects lifetime parameters that are only used once |
| [`trivial_casts`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#trivial-casts) | detects trivial casts which could be removed |
| [`trivial_numeric_casts`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#trivial-numeric-casts) | detects trivial casts of numeric types which could be removed |
| [`unit_bindings`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unit-bindings) | binding is useless because it has the unit `()` type |
| [`unnameable_types`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unnameable-types) | effective visibility of a type is larger than the area in which it can be named |
| [`unreachable_pub`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unreachable-pub) | `pub` items not reachable from crate root |
| [`unsafe_code`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unsafe-code) | usage of `unsafe` code and other potentially unsound constructs |
| [`unstable_features`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unstable-features) | enabling unstable features |
| [`unused_crate_dependencies`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unused-crate-dependencies) | crate dependencies that are never used |
| [`unused_extern_crates`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unused-extern-crates) | extern crates that are never used |
| [`unused_import_braces`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unused-import-braces) | unnecessary braces around an imported item |
| [`unused_lifetimes`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unused-lifetimes) | detects lifetime parameters that are never used |
| [`unused_macro_rules`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unused-macro-rules) | detects macro rules that were not used |
| [`unused_qualifications`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unused-qualifications) | detects unnecessarily qualified names |
| [`unused_results`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unused-results) | unused result of an expression in a statement |
| [`variant_size_differences`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#variant-size-differences) | detects enums with widely varying variant sizes |
