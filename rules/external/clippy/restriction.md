# clippy restriction

Every lint of the clippy `restriction` group as of Rust 1.99, with its level in the preset.

| Lint | Preset | Description |
| --- | --- | --- |
| [`absolute_paths`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#absolute_paths) | warn | checks for usage of an item without a `use` statement |
| [`alloc_instead_of_core`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#alloc_instead_of_core) | warn | type is imported from alloc when available in core |
| [`allow_attributes`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#allow_attributes) | warn | `#[allow]` will not trigger if a warning isn't found. `#[expect]` triggers if there are no warnings. |
| [`allow_attributes_without_reason`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#allow_attributes_without_reason) | warn | ensures that all `allow` and `expect` attributes have a reason |
| [`arbitrary_source_item_ordering`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#arbitrary_source_item_ordering) | allow | arbitrary source item ordering |
| [`arithmetic_side_effects`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#arithmetic_side_effects) | deny | any arithmetic expression that can cause side effects like overflows or panics |
| [`as_conversions`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#as_conversions) | warn | using a potentially dangerous silent `as` conversion |
| [`as_pointer_underscore`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#as_pointer_underscore) | warn | detects `as *mut _` and `as *const _` conversion |
| [`as_underscore`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#as_underscore) | warn | detects `as _` conversion |
| [`assertions_on_result_states`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#assertions_on_result_states) | allow | `assert!(r.is_ok())` or `assert!(r.is_err())` gives worse panic messages than directly calling `r.unwrap()` or `r.unwrap_err()` |
| [`big_endian_bytes`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#big_endian_bytes) | allow | disallows usage of the `to_be_bytes` method |
| [`cfg_not_test`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#cfg_not_test) | warn | enforce against excluding code from test builds |
| [`clone_on_ref_ptr`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#clone_on_ref_ptr) | allow | using `clone` on a ref-counted pointer |
| [`cognitive_complexity`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#cognitive_complexity) | warn | functions that should be split up into multiple functions |
| [`create_dir`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#create_dir) | warn | calling `std::fs::create_dir` instead of `std::fs::create_dir_all` |
| [`dbg_macro`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#dbg_macro) | warn | `dbg!` macro is intended as a debugging tool |
| [`decimal_literal_representation`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#decimal_literal_representation) | warn | using decimal representation when hexadecimal would be better |
| [`default_numeric_fallback`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#default_numeric_fallback) | allow | usage of unconstrained numeric literals which may cause default numeric fallback. |
| [`default_union_representation`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#default_union_representation) | warn | unions without a `#[repr(C)]` attribute |
| [`definition_in_module_root`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#definition_in_module_root) | warn | definitions in `mod.rs` should be in named files |
| [`deref_by_slicing`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#deref_by_slicing) | warn | slicing instead of dereferencing |
| [`disallowed_script_idents`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#disallowed_script_idents) | warn | usage of non-allowed Unicode scripts |
| [`doc_include_without_cfg`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#doc_include_without_cfg) | warn | check if files included in documentation are behind `cfg(doc)` |
| [`doc_paragraphs_missing_punctuation`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#doc_paragraphs_missing_punctuation) | warn | missing terminal punctuation in doc comments |
| [`else_if_without_else`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#else_if_without_else) | warn | `if` expression with an `else if`, but without a final `else` branch |
| [`empty_drop`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#empty_drop) | warn | empty `Drop` implementations |
| [`empty_enum_variants_with_brackets`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#empty_enum_variants_with_brackets) | warn | finds enum variants with empty brackets |
| [`empty_structs_with_brackets`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#empty_structs_with_brackets) | warn | finds struct declarations with empty brackets |
| [`error_impl_error`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#error_impl_error) | warn | exported types named `Error` that implement `Error` |
| [`exhaustive_enums`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#exhaustive_enums) | allow | detects exported enums that have not been marked #[non_exhaustive] |
| [`exhaustive_structs`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#exhaustive_structs) | allow | detects exported structs that have not been marked #[non_exhaustive] |
| [`exit`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#exit) | warn | detects `std::process::exit` calls outside of `main` |
| [`expect_used`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#expect_used) | deny | using `.expect()` on `Result` or `Option`, which might be better handled |
| [`field_scoped_visibility_modifiers`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#field_scoped_visibility_modifiers) | warn | checks for usage of a scoped visibility modifier, like `pub(crate)`, on fields |
| [`filetype_is_file`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#filetype_is_file) | warn | `FileType::is_file` is not recommended to test for readable file type |
| [`float_arithmetic`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#float_arithmetic) | allow | any floating-point arithmetic statement |
| [`float_cmp_const`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#float_cmp_const) | warn | using `==` or `!=` on float constants instead of comparing difference with an allowed error |
| [`fn_to_numeric_cast_any`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#fn_to_numeric_cast_any) | warn | casting a function pointer to any integer type |
| [`get_unwrap`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#get_unwrap) | warn | using `.get().unwrap()` or `.get_mut().unwrap()` when using `[]` would work instead |
| [`host_endian_bytes`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#host_endian_bytes) | warn | disallows usage of the `to_ne_bytes` method |
| [`if_then_some_else_none`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#if_then_some_else_none) | warn | Finds if-else that could be written using either `bool::then` or `bool::then_some` |
| [`impl_trait_in_params`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#impl_trait_in_params) | allow | `impl Trait` is used in the function's parameters |
| [`implicit_return`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#implicit_return) | allow | use a return statement like `return expr` instead of an expression |
| [`indexing_slicing`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#indexing_slicing) | deny | indexing/slicing usage |
| [`infinite_loop`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#infinite_loop) | warn | possibly unintended infinite loop |
| [`inline_asm_x86_att_syntax`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#inline_asm_x86_att_syntax) | warn | prefer Intel x86 assembly syntax |
| [`inline_asm_x86_intel_syntax`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#inline_asm_x86_intel_syntax) | warn | prefer AT&T x86 assembly syntax |
| [`inline_modules`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#inline_modules) | warn | checks that module layout does not use inline modules |
| [`inline_trait_bounds`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#inline_trait_bounds) | allow | enforce that `where` bounds are used for all trait bounds |
| [`integer_division`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#integer_division) | warn | integer division may cause loss of precision |
| [`integer_division_remainder_used`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#integer_division_remainder_used) | allow | use of disallowed default division and remainder operations |
| [`iter_over_hash_type`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#iter_over_hash_type) | warn | iterating over unordered hash-based types (`HashMap` and `HashSet`) |
| [`large_include_file`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#large_include_file) | warn | including a large file |
| [`let_underscore_must_use`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#let_underscore_must_use) | allow | non-binding `let` on a `#[must_use]` expression |
| [`let_underscore_untyped`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#let_underscore_untyped) | allow | non-binding `let` without a type annotation |
| [`little_endian_bytes`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#little_endian_bytes) | warn | disallows usage of the `to_le_bytes` method |
| [`lossy_float_literal`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#lossy_float_literal) | warn | lossy whole number float literals |
| [`map_err_ignore`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#map_err_ignore) | warn | `map_err` should not ignore the original error |
| [`map_with_unused_argument_over_ranges`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#map_with_unused_argument_over_ranges) | warn | map of a trivial closure (not dependent on parameter) over a range |
| [`mem_forget`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#mem_forget) | warn | `mem::forget` usage on `Drop` types, likely to cause memory leaks |
| [`min_ident_chars`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#min_ident_chars) | warn | disallows idents that are too short |
| [`missing_assert_message`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#missing_assert_message) | warn | checks assertions without a custom panic message |
| [`missing_asserts_for_indexing`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#missing_asserts_for_indexing) | warn | indexing into a slice multiple times without an `assert` |
| [`missing_docs_in_private_items`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#missing_docs_in_private_items) | allow | detects missing documentation for private members |
| [`missing_inline_in_public_items`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#missing_inline_in_public_items) | allow | detects missing `#[inline]` attribute for public callables (functions, trait methods, methods...) |
| [`missing_trait_methods`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#missing_trait_methods) | allow | trait implementation uses default provided method |
| [`mixed_read_write_in_expression`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#mixed_read_write_in_expression) | warn | whether a variable read occurs before a write depends on sub-expression evaluation order |
| [`mod_module_files`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#mod_module_files) | deny | checks that module layout is consistent |
| [`module_name_repetitions`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#module_name_repetitions) | warn | type names prefixed/postfixed with their containing module's name |
| [`modulo_arithmetic`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#modulo_arithmetic) | warn | any modulo arithmetic statement |
| [`multiple_inherent_impl`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#multiple_inherent_impl) | warn | Multiple inherent impl that could be grouped |
| [`multiple_unsafe_ops_per_block`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#multiple_unsafe_ops_per_block) | warn | more than one unsafe operation per `unsafe` block |
| [`mutex_atomic`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#mutex_atomic) | warn | using a mutex where an atomic value could be used instead. |
| [`mutex_integer`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#mutex_integer) | warn | using a mutex for an integer type |
| [`needless_raw_strings`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#needless_raw_strings) | warn | suggests using a string literal when a raw string literal is unnecessary |
| [`non_ascii_literal`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#non_ascii_literal) | allow | using any literal non-ASCII chars in a string literal instead of using the `\u` escape |
| [`non_zero_suggestions`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#non_zero_suggestions) | warn | suggests using `NonZero#` from `u#` or `i#` for more efficient and type-safe conversions |
| [`panic`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#panic) | deny | usage of the `panic!` macro |
| [`panic_in_result_fn`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#panic_in_result_fn) | deny | functions of type `Result<..>` that contain `panic!()` or assertion |
| [`partial_pub_fields`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#partial_pub_fields) | warn | partial fields of a struct are public |
| [`pathbuf_init_then_push`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#pathbuf_init_then_push) | warn | `push` immediately after `PathBuf` creation |
| [`pattern_type_mismatch`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#pattern_type_mismatch) | allow | type of pattern does not match the expression type |
| [`pointer_format`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#pointer_format) | warn | formatting a pointer |
| [`precedence_bits`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#precedence_bits) | warn | operations mixing bit shifting with bit combining/masking |
| [`print_stderr`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#print_stderr) | warn | printing on stderr |
| [`print_stdout`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#print_stdout) | warn | printing on stdout |
| [`pub_use`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#pub_use) | allow | restricts the usage of `pub use` |
| [`pub_with_shorthand`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#pub_with_shorthand) | allow | disallows usage of `pub(<loc>)`, without `in` |
| [`pub_without_shorthand`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#pub_without_shorthand) | warn | disallows usage of `pub(in <loc>)` with `in` |
| [`question_mark_used`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#question_mark_used) | allow | checks if the `?` operator is used |
| [`rc_buffer`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#rc_buffer) | warn | shared ownership of a buffer type |
| [`rc_mutex`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#rc_mutex) | warn | usage of `Rc<Mutex<T>>` |
| [`redundant_test_prefix`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#redundant_test_prefix) | warn | redundant `test_` prefix in test function name |
| [`redundant_type_annotations`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#redundant_type_annotations) | warn | warns about needless / redundant type annotations. |
| [`ref_patterns`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#ref_patterns) | warn | use of a ref pattern, e.g. Some(ref value) |
| [`renamed_function_params`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#renamed_function_params) | warn | renamed function parameters in trait implementation |
| [`rest_pat_in_fully_bound_structs`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#rest_pat_in_fully_bound_structs) | warn | a match on a struct that binds all fields but still uses the wildcard pattern |
| [`rest_pattern_accessible_field`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#rest_pattern_accessible_field) | allow | rest pattern (`..`) used for accessible field |
| [`return_and_then`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#return_and_then) | warn | using `Option::and_then` or `Result::and_then` to chain a computation that returns an `Option` or a `Result` |
| [`same_name_method`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#same_name_method) | warn | two method with same name |
| [`self_named_module_files`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#self_named_module_files) | allow | checks that module layout is consistent |
| [`semicolon_inside_block`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#semicolon_inside_block) | warn | add a semicolon inside the block |
| [`semicolon_outside_block`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#semicolon_outside_block) | allow | add a semicolon outside the block |
| [`separated_literal_suffix`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#separated_literal_suffix) | allow | literals whose suffix is separated by an underscore |
| [`shadow_reuse`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#shadow_reuse) | warn | rebinding a name to an expression that reuses the original value, e.g., `let x = x + 1` |
| [`shadow_same`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#shadow_same) | warn | rebinding a name to itself, e.g., `let mut x = &mut x` |
| [`shadow_unrelated`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#shadow_unrelated) | warn | rebinding a name without even using the original value |
| [`single_call_fn`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#single_call_fn) | allow | checks for functions that are only used once |
| [`single_char_lifetime_names`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#single_char_lifetime_names) | warn | warns against single-character lifetime names |
| [`std_instead_of_alloc`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#std_instead_of_alloc) | allow | type is imported from std when available in alloc |
| [`std_instead_of_core`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#std_instead_of_core) | allow | type is imported from std when available in core |
| [`str_to_string`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#str_to_string) | warn | using `to_string()` on a `&str`, which should be `to_owned()` |
| [`string_add`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#string_add) | warn | using `x + ..` where x is a `String` instead of `push_str()` |
| [`string_lit_chars_any`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#string_lit_chars_any) | warn | checks for `<string_lit>.chars().any(|i| i == c)` |
| [`string_slice`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#string_slice) | deny | slicing a string |
| [`suspicious_xor_used_as_pow`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#suspicious_xor_used_as_pow) | warn | XOR (`^`) operator possibly used as exponentiation operator |
| [`tests_outside_test_module`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#tests_outside_test_module) | warn | A test function is outside the testing module. |
| [`todo`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#todo) | deny | `todo!` should not be present in production code |
| [`try_err`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#try_err) | warn | return errors explicitly rather than hiding them behind a `?` |
| [`undocumented_unsafe_blocks`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#undocumented_unsafe_blocks) | warn | creating an unsafe block without explaining why it is safe |
| [`unimplemented`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unimplemented) | deny | `unimplemented!` should not be present in production code |
| [`unnecessary_rest_pattern`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unnecessary_rest_pattern) | warn | unnecessary rest pattern (`..`) in destructuring expression |
| [`unnecessary_safety_comment`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unnecessary_safety_comment) | warn | annotating safe code with a safety comment |
| [`unnecessary_safety_doc`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unnecessary_safety_doc) | warn | `pub fn` or `pub trait` with `# Safety` docs |
| [`unnecessary_self_imports`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unnecessary_self_imports) | warn | imports ending in `::{self}`, which can be omitted |
| [`unneeded_field_pattern`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unneeded_field_pattern) | warn | struct fields bound to a wildcard instead of using `..` |
| [`unreachable`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unreachable) | deny | usage of the `unreachable!` macro |
| [`unseparated_literal_suffix`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unseparated_literal_suffix) | warn | literals whose suffix is not separated by an underscore |
| [`unused_result_ok`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unused_result_ok) | warn | Use of `.ok()` to silence `Result`'s `#[must_use]` is misleading. Use `let _ =` instead. |
| [`unused_trait_names`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unused_trait_names) | warn | use items that import a trait but only use it anonymously |
| [`unwrap_in_result`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unwrap_in_result) | warn | functions of type `Result<..>` or `Option`<...> that contain `expect()` or `unwrap()` |
| [`unwrap_used`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#unwrap_used) | deny | using `.unwrap()` on `Result` or `Option`, which should at least get a better message using `expect()` |
| [`use_debug`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#use_debug) | warn | use of `Debug`-based formatting |
| [`verbose_file_reads`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#verbose_file_reads) | warn | use of `File::read_to_end` or `File::read_to_string` |
| [`wildcard_enum_match_arm`](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#wildcard_enum_match_arm) | warn | a wildcard enum match arm using `_` |
