//! proc-macro tests

#[macro_use]
mod utils;
use test_utils::assert_eq_text;
use utils::*;

#[test]
fn test_derive_serialize_proc_macro() {
    assert_expand(
        "serde_derive",
        "Serialize",
        "1.0.106",
        r##"struct Foo {}"##,
        include_str!("fixtures/test_serialize_proc_macro.txt"),
    );
}

#[test]
fn test_derive_serialize_proc_macro_failed() {
    assert_expand(
        "serde_derive",
        "Serialize",
        "1.0.106",
        r##"
    struct {}
"##,
        r##"
SUBTREE $
  IDENT   compile_error 4294967295
  PUNCH   ! [alone] 4294967295
  SUBTREE {} 4294967295
    LITERAL "expected identifier" 4294967295
"##,
    );
}

#[test]
fn test_derive_proc_macro_list() {
    let res = list("serde_derive", "1.0.106").join("\n");

    assert_eq_text!(
        &res,
        r#"Serialize [CustomDerive]
Deserialize [CustomDerive]"#
    );
}
