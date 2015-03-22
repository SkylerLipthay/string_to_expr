# string_to_expr

Compile-time expression evaluation for strings. [Inspired by trws](https://github.com/rust-lang/rust/issues/12249#issuecomment-49827702). Check `tests/tests.rs` for examples.

I found this technique to be necessary to concatenate identifiers in macros. Really handy for generating structs, enums, and functions. One day, Rust will probably be able to parse macros with more flexibility. Until then, I'm using this crazy mess.
