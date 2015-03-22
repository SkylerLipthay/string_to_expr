#![feature(plugin)]
#![plugin(string_to_expr)]

#[test]
fn test_simple_expr() {
    assert_eq!(string_to_expr!("0x10"), 16);
}

#[test]
fn test_fn_def() {
    string_to_expr!("fn times_three(n: i32) -> i32 { n * 3 }");
    assert_eq!(times_three(5), 15);
}

#[test]
fn test_macro() {
    macro_rules! define_foo {
        ($x:ident) => {
            string_to_expr!(concat!("fn foo_", stringify!($x), "() -> i32 { 10 }"));
        }
    }
    define_foo!(bar);
    assert_eq!(foo_bar(), 10);
}
