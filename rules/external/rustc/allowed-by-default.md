# rustc allowed by default

Every rustc lint that is allowed by default as of Rust 1.99, with its level in the preset.

| Lint | Preset | Description |
| --- | --- | --- |
| [`absolute_paths_not_starting_with_crate`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#absolute-paths-not-starting-with-crate) | allow | fully qualified paths that start with a module name instead of `crate`, `self`, or an extern crate name |
| [`ambiguous_negative_literals`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#ambiguous-negative-literals) | warn | ambiguous negative literals operations |
| [`closure_returning_async_block`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#closure-returning-async-block) | warn | closure that returns `async {}` could be rewritten as an async closure |
| [`dead_code_pub_in_binary`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#dead-code-pub-in-binary) | allow | detect public items in executable crates that are never used |
| [`deprecated_in_future`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#deprecated-in-future) | allow | detects use of items that will be deprecated in a future version |
| [`deprecated_llvm_intrinsic`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#deprecated-llvm-intrinsic) | allow | detects uses of deprecated LLVM intrinsics |
| [`deprecated_safe_2024`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#deprecated-safe-2024) | allow | detects unsafe functions being used as safe functions |
| [`deref_into_dyn_supertrait`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#deref-into-dyn-supertrait) | warn | `Deref` implementation with a supertrait trait object for output is shadowed by trait upcasting |
| [`edition_2024_expr_fragment_specifier`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#edition-2024-expr-fragment-specifier) | allow | The `expr` fragment specifier will accept more expressions in the 2024 edition. To keep the existing behavior, use the `expr_2021` fragment specifier. |
| [`elided_lifetimes_in_paths`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#elided-lifetimes-in-paths) | allow | hidden lifetime parameters in types are deprecated |
| [`explicit_outlives_requirements`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#explicit-outlives-requirements) | warn | outlives requirements can be inferred |
| [`ffi_unwind_calls`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#ffi-unwind-calls) | warn | call to foreign functions or function pointers with FFI-unwind ABI |
| [`if_let_rescope`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#if-let-rescope) | allow | `if let` assigns a shorter lifetime to temporary values being pattern-matched against in Edition 2024 and rewriting in `match` is an option to preserve the semantics up to Edition 2021 |
| [`implicit_provenance_casts`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#implicit-provenance-casts) | allow | an `as` cast relying on exposed provenance is used |
| [`impl_trait_overcaptures`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#impl-trait-overcaptures) | allow | `impl Trait` will capture more lifetimes than possibly intended in edition 2024 |
| [`impl_trait_redundant_captures`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#impl-trait-redundant-captures) | warn | redundant precise-capturing `use<...>` syntax on an `impl Trait` |
| [`keyword_idents_2018`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#keyword-idents-2018) | allow | detects edition keywords being used as an identifier |
| [`keyword_idents_2024`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#keyword-idents-2024) | allow | detects edition keywords being used as an identifier |
| [`let_underscore_drop`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#let-underscore-drop) | warn | non-binding let on a type that has a destructor |
| [`linker_info`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#linker-info) | allow | linker warnings known to be informational-only and not indicative of a problem |
| [`macro_use_extern_crate`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#macro-use-extern-crate) | warn | the `#[macro_use]` attribute is now deprecated in favor of using macros via the module system |
| [`meta_variable_misuse`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#meta-variable-misuse) | warn | possible meta-variable misuse at macro definition |
| [`missing_copy_implementations`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#missing-copy-implementations) | warn | detects potentially-forgotten implementations of `Copy` |
| [`missing_debug_implementations`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#missing-debug-implementations) | allow | detects missing implementations of Debug |
| [`missing_docs`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#missing-docs) | warn | detects missing documentation for public members |
| [`missing_unsafe_on_extern`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#missing-unsafe-on-extern) | allow | detects missing unsafe keyword on extern declarations |
| [`multiple_supertrait_upcastable`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#multiple-supertrait-upcastable) | allow | detect when a dyn-compatible trait has multiple supertraits |
| [`must_not_suspend`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#must-not-suspend) | allow | use of a `#[must_not_suspend]` value across a yield point |
| [`non_ascii_idents`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#non-ascii-idents) | warn | detects non-ASCII identifiers |
| [`non_exhaustive_omitted_patterns`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#non-exhaustive-omitted-patterns) | allow | detect when patterns of types marked `non_exhaustive` are missed |
| [`raw_borrows_via_references`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#raw-borrows-via-references) | warn | creating raw borrows via references is discouraged |
| [`redundant_imports`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#redundant-imports) | warn | imports that are redundant due to being imported already |
| [`redundant_lifetimes`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#redundant-lifetimes) | warn | detects lifetime parameters that are redundant because they are equal to some other named lifetime |
| [`resolving_to_items_shadowing_supertrait_items`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#resolving-to-items-shadowing-supertrait-items) | allow | detects when a supertrait item is shadowed by a subtrait item |
| [`rust_2021_incompatible_closure_captures`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#rust-2021-incompatible-closure-captures) | allow | detects closures affected by Rust 2021 changes |
| [`rust_2021_incompatible_or_patterns`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#rust-2021-incompatible-or-patterns) | allow | detects usage of old versions of or-patterns |
| [`rust_2021_prefixes_incompatible_syntax`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#rust-2021-prefixes-incompatible-syntax) | allow | identifiers that will be parsed as a prefix in Rust 2021 |
| [`rust_2021_prelude_collisions`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#rust-2021-prelude-collisions) | allow | detects the usage of trait methods which are ambiguous with traits added to the prelude in future editions |
| [`rust_2024_guarded_string_incompatible_syntax`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#rust-2024-guarded-string-incompatible-syntax) | allow | will be parsed as a guarded string in Rust 2024 |
| [`rust_2024_incompatible_pat`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#rust-2024-incompatible-pat) | allow | detects patterns whose meaning will change in Rust 2024 |
| [`rust_2024_prelude_collisions`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#rust-2024-prelude-collisions) | allow | detects the usage of trait methods which are ambiguous with traits added to the prelude in future editions |
| [`shadowing_supertrait_items`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#shadowing-supertrait-items) | allow | detects when a supertrait item is shadowed by a subtrait item |
| [`single_use_lifetimes`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#single-use-lifetimes) | warn | detects lifetime parameters that are only used once |
| [`tail_expr_drop_order`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#tail-expr-drop-order) | allow | Detect and warn on significant change in drop order in tail expression location |
| [`trivial_casts`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#trivial-casts) | warn | detects trivial casts which could be removed |
| [`trivial_numeric_casts`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#trivial-numeric-casts) | warn | detects trivial casts of numeric types which could be removed |
| [`unit_bindings`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unit-bindings) | warn | binding is useless because it has the unit `()` type |
| [`unnameable_types`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unnameable-types) | warn | effective visibility of a type is larger than the area in which it can be named |
| [`unqualified_local_imports`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unqualified-local-imports) | allow | `use` of a local item without leading `self::`, `super::`, or `crate::` |
| [`unreachable_pub`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unreachable-pub) | warn | `pub` items not reachable from crate root |
| [`unsafe_attr_outside_unsafe`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unsafe-attr-outside-unsafe) | allow | detects unsafe attributes outside of unsafe |
| [`unsafe_code`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unsafe-code) | deny | usage of `unsafe` code and other potentially unsound constructs |
| [`unsafe_op_in_unsafe_fn`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unsafe-op-in-unsafe-fn) | allow | unsafe operations in unsafe functions without an explicit unsafe block are deprecated |
| [`unstable_features`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unstable-features) | warn | enabling unstable features |
| [`unused_crate_dependencies`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unused-crate-dependencies) | allow | crate dependencies that are never used |
| [`unused_extern_crates`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unused-extern-crates) | warn | extern crates that are never used |
| [`unused_import_braces`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unused-import-braces) | warn | unnecessary braces around an imported item |
| [`unused_lifetimes`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unused-lifetimes) | warn | detects lifetime parameters that are never used |
| [`unused_macro_rules`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unused-macro-rules) | warn | detects macro rules that were not used |
| [`unused_qualifications`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unused-qualifications) | warn | detects unnecessarily qualified names |
| [`unused_results`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#unused-results) | warn | unused result of an expression in a statement |
| [`variant_size_differences`](https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html#variant-size-differences) | warn | detects enums with widely varying variant sizes |
