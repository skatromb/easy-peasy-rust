# clippy nursery

Every lint of the clippy `nursery` group as of Rust 1.99. All are allowed by default.

| Lint | Description |
| --- | --- |
| [`as_ptr_cast_mut`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#as_ptr_cast_mut) | casting the result of the `&self`-taking `as_ptr` to a mutable pointer |
| [`branches_sharing_code`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#branches_sharing_code) | `if` statement with shared code in all blocks |
| [`clear_with_drain`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#clear_with_drain) | calling `drain` in order to `clear` a container |
| [`coerce_container_to_any`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#coerce_container_to_any) | coercing to `&dyn Any` when dereferencing could produce a `dyn Any` without coercion is usually not intended |
| [`collection_is_never_read`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#collection_is_never_read) | a collection is never queried |
| [`debug_assert_with_mut_call`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#debug_assert_with_mut_call) | mutable arguments in `debug_assert{,_ne,_eq}!` |
| [`derive_partial_eq_without_eq`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#derive_partial_eq_without_eq) | deriving `PartialEq` on a type that can implement `Eq`, without implementing `Eq` |
| [`doc_link_code`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#doc_link_code) | link with code back-to-back with other code |
| [`empty_enums`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#empty_enums) | enum with no variants |
| [`equatable_if_let`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#equatable_if_let) | using pattern matching instead of equality |
| [`fallible_impl_from`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#fallible_impl_from) | Warn on impls of `From<..>` that contain `panic!()` or `unwrap()` |
| [`future_not_send`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#future_not_send) | public Futures must be Send |
| [`imprecise_flops`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#imprecise_flops) | usage of imprecise floating point operations |
| [`iter_on_empty_collections`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#iter_on_empty_collections) | Iterator for empty array |
| [`iter_on_single_items`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#iter_on_single_items) | Iterator for array of length 1 |
| [`iter_with_drain`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#iter_with_drain) | replace `.drain(..)` with `.into_iter()` |
| [`large_stack_frames`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#large_stack_frames) | checks for functions that allocate a lot of stack space |
| [`literal_string_with_formatting_args`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#literal_string_with_formatting_args) | Checks if string literals have formatting arguments |
| [`missing_const_for_fn`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#missing_const_for_fn) | Lint functions definitions that could be made `const fn` |
| [`needless_collect`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#needless_collect) | collecting an iterator when collect is not needed |
| [`needless_pass_by_ref_mut`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#needless_pass_by_ref_mut) | using a `&mut` argument when it's not mutated |
| [`needless_type_cast`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#needless_type_cast) | binding defined with one type but always cast to another |
| [`non_send_fields_in_send_ty`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#non_send_fields_in_send_ty) | there is a field that is not safe to be sent to another thread in a `Send` struct |
| [`option_if_let_else`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#option_if_let_else) | reimplementation of Option::map_or |
| [`or_fun_call`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#or_fun_call) | using any `*or` method with a function call, which suggests `*or_else` |
| [`path_buf_push_overwrite`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#path_buf_push_overwrite) | calling `push` with file system root on `PathBuf` can overwrite it |
| [`read_zero_byte_vec`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#read_zero_byte_vec) | checks for reads into a zero-length `Vec` |
| [`redundant_clone`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#redundant_clone) | `clone()` of an owned value that is going to be dropped immediately |
| [`redundant_pub_crate`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#redundant_pub_crate) | Using `pub(crate)` visibility on items that are not crate visible due to the visibility of the module that contains them. |
| [`search_is_some`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#search_is_some) | using an iterator or string search followed by `is_some()` or `is_none()`, which is more succinctly expressed as a call to `any()` or `contains()` (with negation in case of `is_none()`) |
| [`set_contains_or_insert`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#set_contains_or_insert) | call to `<set>::contains` followed by `<set>::insert` |
| [`significant_drop_in_scrutinee`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#significant_drop_in_scrutinee) | warns when a temporary of a type with a drop with a significant side-effect might have a surprising lifetime |
| [`significant_drop_tightening`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#significant_drop_tightening) | Searches for elements marked with `#[clippy::has_significant_drop]` that could be early dropped but are in fact dropped at the end of their scopes |
| [`single_option_map`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#single_option_map) | Checks for functions with method calls to `.map(_)` on an arg of type `Option` as the outermost expression. |
| [`string_lit_as_bytes`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#string_lit_as_bytes) | calling `as_bytes` on a string literal instead of using a byte string literal |
| [`suboptimal_flops`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#suboptimal_flops) | usage of sub-optimal floating point operations |
| [`suspicious_operation_groupings`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#suspicious_operation_groupings) | groupings of binary operations that look suspiciously like typos |
| [`too_long_first_doc_paragraph`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#too_long_first_doc_paragraph) | ensure the first documentation paragraph is short |
| [`trailing_empty_array`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#trailing_empty_array) | struct with a trailing zero-sized array but without `#[repr(C)]` or another `repr` attribute |
| [`trait_duplication_in_bounds`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#trait_duplication_in_bounds) | check if the same trait bounds are specified more than once during a generic declaration |
| [`transmute_undefined_repr`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#transmute_undefined_repr) | transmute to or from a type with an undefined representation |
| [`trivial_regex`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#trivial_regex) | trivial regular expressions |
| [`tuple_array_conversions`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#tuple_array_conversions) | checks for tuple<=>array conversions that are not done with `.into()` |
| [`type_repetition_in_bounds`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#type_repetition_in_bounds) | types are repeated unnecessarily in trait bounds, use `+` instead of using `T: _, T: _` |
| [`uninhabited_references`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#uninhabited_references) | reference to uninhabited type |
| [`unnecessary_struct_initialization`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unnecessary_struct_initialization) | struct built from a base that can be written mode concisely |
| [`unused_peekable`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unused_peekable) | creating a peekable iterator without using any of its methods |
| [`unused_rounding`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unused_rounding) | Uselessly rounding a whole number floating-point literal |
| [`use_self`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#use_self) | unnecessary structure name repetition whereas `Self` is applicable |
| [`useless_let_if_seq`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#useless_let_if_seq) | unidiomatic `let mut` declaration followed by initialization in `if` |
| [`volatile_composites`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#volatile_composites) | warn about volatile read/write applied to composite types |
| [`while_float`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#while_float) | while loops comparing floating point values |
