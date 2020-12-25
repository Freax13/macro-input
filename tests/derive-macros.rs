use macro_input::MacroInput;
use std::{convert::TryFrom, fmt::Debug};
use syn::{parse_quote, Attribute};

#[derive(MacroInput, PartialEq, Debug)]
pub struct Input {
    pub flag: Option<()>,
    pub optional: Option<i32>,
    #[macro_input(default_value = 3)]
    pub with_default: i32,
    pub required: i32,
}

#[test]
fn test_basic_input() {
    fn test_input(attr: Attribute, value: Input) {
        let attrs = vec![attr];

        let res = Input::try_from(attrs.as_slice()).unwrap();
        assert_eq!(value, res);
    }

    test_input(
        parse_quote!(#[input(flag, optional = 1, with_default = 2, required = 3)]),
        Input {
            flag: Some(()),
            optional: Some(1),
            with_default: 2,
            required: 3,
        },
    );

    test_input(
        parse_quote!(#[input(required = 3)]),
        Input {
            flag: None,
            optional: None,
            with_default: 3,
            required: 3,
        },
    );
}

#[derive(MacroInput, PartialEq, Debug)]
#[macro_input(rename = "yet_another_name")]
pub struct OtherInput {
    #[macro_input(rename = "new_name")]
    pub renamed: i32,
}

#[test]
fn test_other_input() {
    fn test_input(attr: Attribute, value: OtherInput) {
        let attrs = vec![attr];

        let res = OtherInput::try_from(attrs.as_slice()).unwrap();
        assert_eq!(value, res);
    }

    test_input(
        parse_quote!(#[yet_another_name(new_name = 3)]),
        OtherInput { renamed: 3 },
    );
}
