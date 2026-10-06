# clippy pedantic

Every lint of the clippy `pedantic` group as of Rust 1.99, with its level in the preset.

| Lint | Preset | Description |
| --- | --- | --- |
| [`assert_is_empty`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#assert_is_empty) | warn | asserting on emptiness without showing the asserted value on failure |
| [`assigning_clones`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#assigning_clones) | warn | assigning the result of cloning may be inefficient |
| [`bool_to_int_with_if`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#bool_to_int_with_if) | warn | using if to convert bool to int |
| [`borrow_as_ptr`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#borrow_as_ptr) | warn | borrowing just to cast to a raw pointer |
| [`case_sensitive_file_extension_comparisons`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#case_sensitive_file_extension_comparisons) | warn | Checks for calls to ends_with with case-sensitive file extensions |
| [`cast_lossless`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#cast_lossless) | warn | casts using `as` that are known to be lossless, e.g., `x as u64` where `x: u8` |
| [`cast_possible_truncation`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#cast_possible_truncation) | warn | casts that may cause truncation of the value, e.g., `x as u8` where `x: u32`, or `x as i32` where `x: f32` |
| [`cast_possible_wrap`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#cast_possible_wrap) | warn | casts that may cause wrapping around the value, e.g., `x as i32` where `x: u32` and `x > i32::MAX` |
| [`cast_precision_loss`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#cast_precision_loss) | warn | casts that cause loss of precision, e.g., `x as f32` where `x: u64` |
| [`cast_ptr_alignment`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#cast_ptr_alignment) | warn | cast from a pointer to a more strictly aligned pointer |
| [`cast_sign_loss`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#cast_sign_loss) | warn | casts from signed types to unsigned types, e.g., `x as u32` where `x: i32` |
| [`checked_conversions`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#checked_conversions) | warn | `try_from` could replace manual bounds checking when casting |
| [`cloned_instead_of_copied`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#cloned_instead_of_copied) | warn | used `cloned` where `copied` could be used instead |
| [`collapsible_else_if`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#collapsible_else_if) | warn | nested `else`-`if` expressions that can be collapsed (e.g., `else { if x { ... } }`) |
| [`comparison_chain`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#comparison_chain) | warn | `if`s that can be rewritten with `match` and `cmp` |
| [`copy_iterator`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#copy_iterator) | warn | implementing `Iterator` on a `Copy` type |
| [`decimal_bitwise_operands`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#decimal_bitwise_operands) | warn | use binary, hex, or octal literals for bitwise operations |
| [`default_trait_access`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#default_trait_access) | warn | checks for literal calls to `Default::default()` |
| [`doc_broken_link`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#doc_broken_link) | warn | broken document link |
| [`doc_comment_double_space_linebreaks`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#doc_comment_double_space_linebreaks) | warn | double space used for doc comment hard line break instead of `\` |
| [`doc_link_with_quotes`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#doc_link_with_quotes) | warn | possible typo for an intra-doc link |
| [`doc_markdown`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#doc_markdown) | warn | presence of `_`, `::` or camel-case outside backticks in documentation |
| [`duration_suboptimal_units`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#duration_suboptimal_units) | warn | constructing a `Duration` using a smaller unit when a larger unit would be more readable |
| [`elidable_lifetime_names`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#elidable_lifetime_names) | warn | lifetime name that can be replaced with the anonymous lifetime |
| [`enum_glob_use`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#enum_glob_use) | warn | use items that import all variants of an enum |
| [`expl_impl_clone_on_copy`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#expl_impl_clone_on_copy) | warn | implementing `Clone` explicitly on `Copy` types |
| [`explicit_deref_methods`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#explicit_deref_methods) | warn | Explicit use of deref or deref_mut method while not in a method chain. |
| [`explicit_into_iter_loop`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#explicit_into_iter_loop) | warn | for-looping over `_.into_iter()` when `_` would do |
| [`explicit_iter_loop`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#explicit_iter_loop) | warn | for-looping over `_.iter()` or `_.iter_mut()` when `&_` or `&mut _` would do |
| [`filter_map_next`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#filter_map_next) | warn | using combination of `filter_map` and `next` which can usually be written as a single method call |
| [`flat_map_option`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#flat_map_option) | warn | used `flat_map` where `filter_map` could be used instead |
| [`float_cmp`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#float_cmp) | warn | using `==` or `!=` on float values instead of comparing difference with an allowed error |
| [`fn_params_excessive_bools`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#fn_params_excessive_bools) | warn | using too many bools in function parameters |
| [`format_collect`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#format_collect) | warn | `format!`ing every element in a collection, then collecting the strings into a new `String` |
| [`format_push_string`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#format_push_string) | warn | `format!(..)` appended to existing `String` |
| [`if_not_else`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#if_not_else) | warn | `if` branches that could be swapped so no negation operation is necessary on the condition |
| [`ignore_without_reason`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#ignore_without_reason) | warn | ignored tests without messages |
| [`ignored_unit_patterns`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#ignored_unit_patterns) | warn | suggest replacing `_` by `()` in patterns where appropriate |
| [`implicit_clone`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#implicit_clone) | warn | implicitly cloning a value by invoking a function on its dereferenced type |
| [`implicit_hasher`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#implicit_hasher) | warn | missing generalization over different hashers |
| [`inconsistent_struct_constructor`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#inconsistent_struct_constructor) | warn | the order of the field init is inconsistent with the order in the struct definition |
| [`index_refutable_slice`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#index_refutable_slice) | warn | avoid indexing on slices which could be destructed |
| [`inefficient_to_string`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#inefficient_to_string) | warn | using `to_string` on `&&T` where `T: ToString` |
| [`inline_always`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#inline_always) | warn | use of `#[inline(always)]` |
| [`into_iter_without_iter`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#into_iter_without_iter) | warn | implementing `IntoIterator for (&|&mut) Type` without an inherent `iter(_mut)` method |
| [`invalid_upcast_comparisons`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#invalid_upcast_comparisons) | warn | a comparison involving an upcast which is always true or false |
| [`ip_constant`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#ip_constant) | warn | hardcoded localhost IP address |
| [`items_after_statements`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#items_after_statements) | warn | blocks where an item comes after a statement |
| [`iter_filter_is_ok`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#iter_filter_is_ok) | warn | filtering an iterator over `Result`s for `Ok` can be achieved with `flatten` |
| [`iter_filter_is_some`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#iter_filter_is_some) | warn | filtering an iterator over `Option`s for `Some` can be achieved with `flatten` |
| [`iter_not_returning_iterator`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#iter_not_returning_iterator) | warn | methods named `iter` or `iter_mut` that do not return an `Iterator` |
| [`iter_without_into_iter`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#iter_without_into_iter) | warn | implementing `iter(_mut)` without an associated `IntoIterator for (&|&mut) Type` impl |
| [`large_digit_groups`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#large_digit_groups) | warn | grouping digits into groups that are too large |
| [`large_futures`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#large_futures) | warn | large future may lead to unexpected stack overflows |
| [`large_stack_arrays`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#large_stack_arrays) | warn | allocating large arrays on stack may cause stack overflow |
| [`large_types_passed_by_value`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#large_types_passed_by_value) | warn | functions taking large arguments by value |
| [`linkedlist`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#linkedlist) | warn | usage of LinkedList, usually a vector is faster, or a more specialized data structure like a `VecDeque` |
| [`macro_use_imports`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#macro_use_imports) | warn | #[macro_use] is no longer needed |
| [`manual_assert`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#manual_assert) | warn | `panic!` and only a `panic!` in `if`-then statement |
| [`manual_assert_eq`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#manual_assert_eq) | warn | checks for assertions consisting of an (in)equality check |
| [`manual_bit_width`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#manual_bit_width) | warn | manually reimplementing `bit_width` |
| [`manual_ilog2`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#manual_ilog2) | warn | manually reimplementing `ilog2` |
| [`manual_instant_elapsed`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#manual_instant_elapsed) | warn | subtraction between `Instant::now()` and previous `Instant` |
| [`manual_is_power_of_two`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#manual_is_power_of_two) | warn | manually reimplementing `is_power_of_two` |
| [`manual_is_variant_and`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#manual_is_variant_and) | warn | using `.map(f).unwrap_or_default()` or `.map(f) == Some/Ok(true)`, which are more succinctly expressed as `is_some_and(f)` or `is_ok_and(f)` |
| [`manual_let_else`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#manual_let_else) | warn | manual implementation of a let...else statement |
| [`manual_midpoint`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#manual_midpoint) | warn | manual implementation of `midpoint` which can overflow |
| [`manual_string_new`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#manual_string_new) | warn | empty String is being created manually |
| [`many_single_char_names`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#many_single_char_names) | warn | too many single character bindings |
| [`map_unwrap_or`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#map_unwrap_or) | warn | using `.map(f).unwrap_or(a)` or `.map(f).unwrap_or_else(func)`, which are more succinctly expressed as `map_or(a, f)` or `map_or_else(a, f)` |
| [`match_bool`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#match_bool) | warn | a `match` on a boolean expression instead of an `if..else` block |
| [`match_same_arms`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#match_same_arms) | warn | `match` with identical arm bodies |
| [`match_wild_err_arm`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#match_wild_err_arm) | warn | a `match` with `Err(_)` arm and take drastic actions |
| [`match_wildcard_for_single_variants`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#match_wildcard_for_single_variants) | warn | a wildcard enum match for a single variant |
| [`maybe_infinite_iter`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#maybe_infinite_iter) | warn | possible infinite iteration |
| [`mismatching_type_param_order`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#mismatching_type_param_order) | warn | type parameter positioned inconsistently between type def and impl block |
| [`missing_errors_doc`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#missing_errors_doc) | warn | `pub fn` returns `Result` without `# Errors` in doc comment |
| [`missing_fields_in_debug`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#missing_fields_in_debug) | warn | missing fields in manual `Debug` implementation |
| [`missing_panics_doc`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#missing_panics_doc) | warn | `pub fn` may panic without `# Panics` in doc comment |
| [`must_use_candidate`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#must_use_candidate) | warn | function or method that could take a `#[must_use]` attribute |
| [`mut_mut`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#mut_mut) | warn | usage of double mut-refs, e.g., `&mut &mut ...` |
| [`naive_bytecount`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#naive_bytecount) | warn | use of naive `<slice>.filter(|&x| x == y).count()` to count byte values |
| [`needless_bitwise_bool`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#needless_bitwise_bool) | warn | Boolean expressions that use bitwise rather than lazy operators |
| [`needless_continue`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#needless_continue) | warn | `continue` statements that can be replaced by a rearrangement of code |
| [`needless_for_each`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#needless_for_each) | warn | using `for_each` where a `for` loop would be simpler |
| [`needless_pass_by_value`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#needless_pass_by_value) | warn | functions taking arguments by value, but not consuming them in its body |
| [`needless_raw_string_hashes`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#needless_raw_string_hashes) | warn | suggests reducing the number of hashes around a raw string literal |
| [`no_effect_underscore_binding`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#no_effect_underscore_binding) | warn | binding to `_` prefixed variable with no side-effect |
| [`no_mangle_with_rust_abi`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#no_mangle_with_rust_abi) | warn | convert Rust ABI functions to C ABI |
| [`non_std_lazy_statics`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#non_std_lazy_statics) | warn | lazy static that could be replaced by `std::sync::LazyLock` |
| [`nonminimal_bool`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#nonminimal_bool) | warn | boolean expressions that can be written more concisely |
| [`option_as_ref_cloned`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#option_as_ref_cloned) | warn | cloning an `Option` via `as_ref().cloned()` |
| [`option_option`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#option_option) | warn | usage of `Option<Option<T>>` |
| [`overly_complex_bool_expr`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#overly_complex_bool_expr) | warn | boolean expressions that contain terminals which can be eliminated |
| [`ptr_as_ptr`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#ptr_as_ptr) | warn | casting using `as` between raw pointers that doesn't change their constness, where `pointer::cast` could take the place of `as` |
| [`ptr_cast_constness`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#ptr_cast_constness) | warn | casting using `as` on raw pointers to change constness when specialized methods apply |
| [`ptr_offset_by_literal`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#ptr_offset_by_literal) | warn | unneeded pointer offset |
| [`pub_underscore_fields`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#pub_underscore_fields) | warn | struct field prefixed with underscore and marked public |
| [`range_minus_one`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#range_minus_one) | warn | `x..=(y-1)` reads better as `x..y` |
| [`range_plus_one`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#range_plus_one) | warn | `x..(y+1)` reads better as `x..=y` |
| [`redundant_closure_for_method_calls`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#redundant_closure_for_method_calls) | warn | redundant closures for method calls |
| [`redundant_else`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#redundant_else) | warn | `else` branch that can be removed without changing semantics |
| [`ref_as_ptr`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#ref_as_ptr) | warn | using `as` to cast a reference to pointer |
| [`ref_binding_to_reference`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#ref_binding_to_reference) | warn | `ref` binding to a reference |
| [`ref_option`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#ref_option) | warn | function signature uses `&Option<T>` instead of `Option<&T>` |
| [`ref_option_ref`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#ref_option_ref) | warn | use `Option<&T>` instead of `&Option<&T>` |
| [`return_self_not_must_use`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#return_self_not_must_use) | warn | missing `#[must_use]` annotation on a method returning `Self` |
| [`same_functions_in_if_condition`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#same_functions_in_if_condition) | warn | consecutive `if`s with the same function call |
| [`same_length_and_capacity`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#same_length_and_capacity) | warn | `from_raw_parts` with same length and capacity |
| [`self_only_used_in_recursion`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#self_only_used_in_recursion) | warn | self receiver only used to recursively call method can be removed |
| [`semicolon_if_nothing_returned`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#semicolon_if_nothing_returned) | warn | add a semicolon if nothing is returned |
| [`should_panic_without_expect`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#should_panic_without_expect) | warn | ensures that all `should_panic` attributes specify its expected panic message |
| [`similar_names`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#similar_names) | warn | similarly named items and bindings |
| [`single_char_pattern`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#single_char_pattern) | warn | using a single-character str where a char could be used, e.g., `_.split("x")` |
| [`single_match_else`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#single_match_else) | warn | a `match` statement with two arms where the second arm's pattern is a placeholder instead of a specific match pattern |
| [`stable_sort_primitive`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#stable_sort_primitive) | warn | use of sort() when sort_unstable() is equivalent |
| [`str_split_at_newline`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#str_split_at_newline) | warn | splitting a trimmed string at hard-coded newlines |
| [`string_add_assign`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#string_add_assign) | warn | using `x = x + ..` where x is a `String` instead of `push_str()` |
| [`struct_excessive_bools`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#struct_excessive_bools) | warn | using too many bools in a struct |
| [`struct_field_names`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#struct_field_names) | warn | structs where all fields share a prefix/postfix or contain the name of the struct |
| [`too_many_lines`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#too_many_lines) | warn | functions with too many lines |
| [`transmute_ptr_to_ptr`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#transmute_ptr_to_ptr) | warn | transmutes from a pointer to a pointer / a reference to a reference |
| [`trivially_copy_pass_by_ref`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#trivially_copy_pass_by_ref) | warn | functions taking small copyable arguments by reference |
| [`unchecked_time_subtraction`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unchecked_time_subtraction) | deny | finds unchecked subtraction involving 'Duration' or 'Instant' |
| [`unicode_not_nfc`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unicode_not_nfc) | warn | using a Unicode literal not in NFC normal form (see [Unicode tr15](http://www.unicode.org/reports/tr15/) for further information) |
| [`uninlined_format_args`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#uninlined_format_args) | warn | using non-inlined variables in `format!` calls |
| [`unnecessary_box_returns`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unnecessary_box_returns) | warn | Needlessly returning a Box |
| [`unnecessary_debug_formatting`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unnecessary_debug_formatting) | warn | `Debug` formatting applied to an `OsStr` or `Path` when `.display()` is available |
| [`unnecessary_join`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unnecessary_join) | warn | using `.collect::<Vec<String>>().join("")` on an iterator |
| [`unnecessary_literal_bound`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unnecessary_literal_bound) | warn | detects &str that could be &'static str in function return types |
| [`unnecessary_semicolon`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unnecessary_semicolon) | warn | unnecessary semicolon after expression returning `()` |
| [`unnecessary_trailing_comma`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unnecessary_trailing_comma) | warn | unnecessary trailing comma before closing parenthesis |
| [`unnecessary_wraps`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unnecessary_wraps) | warn | functions that only return `Ok` or `Some` |
| [`unnested_or_patterns`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unnested_or_patterns) | warn | unnested or-patterns, e.g., `Foo(Bar) | Foo(Baz) instead of `Foo(Bar | Baz)` |
| [`unreadable_literal`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unreadable_literal) | warn | long literal without underscores |
| [`unsafe_derive_deserialize`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unsafe_derive_deserialize) | warn | deriving `serde::Deserialize` on a type that has methods using `unsafe` |
| [`unused_async`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unused_async) | warn | finds async functions with no await statements |
| [`unused_async_trait_impl`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unused_async_trait_impl) | warn | finds async trait impl functions with no await statements |
| [`unused_self`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unused_self) | warn | methods that contain a `self` argument but don't use it |
| [`used_underscore_binding`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#used_underscore_binding) | warn | using a binding which is prefixed with an underscore |
| [`used_underscore_items`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#used_underscore_items) | warn | using a item which is prefixed with an underscore |
| [`verbose_bit_mask`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#verbose_bit_mask) | warn | expressions where a bit mask is less readable than the corresponding method call |
| [`wildcard_imports`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#wildcard_imports) | warn | lint `use _::*` statements |
| [`with_capacity_zero`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#with_capacity_zero) | warn | calling `with_capacity(0)` which is equivalent to `new()` |
| [`zero_sized_map_values`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#zero_sized_map_values) | warn | usage of map with zero-sized value type |
