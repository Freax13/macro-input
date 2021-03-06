use macro_input::{Def, DefaultValue, FromMeta};
use std::fmt::Debug;
use syn::{parse_quote, Attribute};

#[test]
fn test_string() {
    const FIELD: Def = Def::new("foo", "bar", true, DefaultValue::Str(None));

    test_field::<String>(parse_quote!(#[foo(bar = "baz")]), &FIELD, "baz".to_string());
}

#[test]
fn test_flag() {
    const FIELD: Def = Def::new("foo", "bar", false, DefaultValue::Flag);

    test_field::<Option<()>>(parse_quote!(#[foo(bar)]), &FIELD, Some(()));
    test_field::<Option<()>>(parse_quote!(#[foo(other)]), &FIELD, None);
}

#[test]
fn test_optional_string() {
    const FIELD: Def = Def::new("foo", "bar", false, DefaultValue::Str(None));

    test_field::<Option<String>>(
        parse_quote!(#[foo(bar = "baz")]),
        &FIELD,
        Some("baz".to_string()),
    );
    test_field::<Option<String>>(parse_quote!(#[foo(other = "baz")]), &FIELD, None);
}

#[test]
fn test_default_string() {
    const FIELD: Def = Def::new("foo", "bar", false, DefaultValue::Str(Some("baz")));

    test_field::<String>(parse_quote!(#[foo(bar = "qux")]), &FIELD, "qux".to_string());
    test_field::<String>(
        parse_quote!(#[foo(other = "qux")]),
        &FIELD,
        "baz".to_string(),
    );
}

fn test_field<T: FromMeta + PartialEq + Debug>(attr: Attribute, field: &Def, value: T) {
    let attrs = vec![attr];
    assert_eq!(field.get_value::<T>(&attrs).unwrap(), value);
}
