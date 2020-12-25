mod convert;
mod fielddef;
mod fns;
mod input;
mod lint;

use heck::{ShoutySnekCase, SnekCase};
use input::{DEFAULT_VALUE_FIELD, RENAME_FIELD};
use macro_compose::{Collector, Context};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::format_ident;
use syn::{DeriveInput, Field, LitStr};

#[proc_macro_derive(MacroInput, attributes(macro_input))]
/// automatically derive `TryFrom<&[syn::Attribute]>` and `fn strip(attrs: &mut Vec<syn::Attribute>)`
///
/// supported types:
/// - `u8`, `i32`, `f32`, `char`, `bool`, `String` or `Vec<u8>` for parsing literals eg `#[foo(bar = 3)]`
/// - `Option<u8>`, `Option<i32>`, `Option<f32>`, `Option<char>`, `Option<bool>`, `Option<String>` or `Option<Vec<u8>>` for optionally parsing literals eg either `#[foo(bar = 3)]` or nothing
/// - `Option<()>` for parsing flags eg `#[foo(bar)]`
///
/// paths get converted to lower_snake unless `rename` is specified
///
/// use `#[macro_input]` for customization:
/// - `rename` to rename either the path or field name eg `#[macro_input(rename = "some_name")]`
/// - `default_value` for default values eg `#[macro_input(default_value = "some literal")]`
/// # Example
/// ```
/// use macro_input_macros::MacroInput;
/// use std::convert::TryFrom;
/// use syn::{parse_quote, Attribute};
///
/// #[derive(MacroInput, PartialEq, Debug)]
/// pub struct SomeInput {
///     pub flag: Option<()>,
///     pub optional: Option<i32>,
///     #[macro_input(default_value = 3)]
///     pub with_default: i32,
///     pub required: i32,
/// }
///
/// #[derive(MacroInput, PartialEq, Debug)]
/// #[macro_input(rename = "YetAnotherName")]
/// pub struct OtherInput {
///     #[macro_input(rename = "new_name")]
///     pub renamed: i32,
/// }
///
/// # fn main() -> syn::Result<()> {
/// // construct some attributes
/// let attr1: Attribute = parse_quote!(#[some_input(flag, required = 5)]);
/// let attr2: Attribute = parse_quote!(#[some_input(optional = 8, with_default = 4)]);
/// let attr3: Attribute = parse_quote!(#[YetAnotherName(new_name = 6)]);
///
/// // parse SomeInput with only some attributes
/// let input1 = SomeInput::try_from(&[attr1.clone()] as &[Attribute])?;
/// // parse SomeInput with all attributes
/// let input2 = SomeInput::try_from(&[attr1, attr2] as &[Attribute])?;
/// // parse OtherInput
/// let input3 = OtherInput::try_from(&[attr3] as &[Attribute])?;
///
/// assert_eq!(input1, SomeInput { flag: Some(()), optional: None, with_default: 3, required: 5 });
/// assert_eq!(input2, SomeInput { flag: Some(()), optional: Some(8), with_default: 4, required: 5 });
/// assert_eq!(input3, OtherInput { renamed: 6 });
/// # Ok(())
/// # }
/// ```

pub fn derive_macro_input(item: TokenStream) -> TokenStream {
    let mut collector = Collector::new();
    let mut ctx = Context::<DeriveInput>::new_parse(&mut collector, item);

    // lint
    ctx.lint(&input::STRUCT_LINT);
    ctx.lint(&fielddef::Name);
    ctx.lint(&fielddef::FieldType);
    ctx.lint(&lint::Name);

    // expand
    ctx.expand(&convert::TryFromAttributes);
    ctx.expand(&fielddef::ConstFields);
    ctx.expand(&fns::Strip);

    collector.finish().into()
}

fn mod_name(input: &DeriveInput) -> Ident {
    let path = input.ident.to_string().to_snek_case();
    format_ident!("__{}", &*path, span = input.ident.span())
}

fn field_name(f: &Field) -> (String, Ident) {
    let (name, span) = RENAME_FIELD.get::<LitStr>(&f.attrs).unwrap().map_or_else(
        || {
            let ident = f.ident.as_ref().unwrap();
            (ident.to_string(), ident.span())
        },
        |s| (s.value(), s.span()),
    );
    let field_name = format!("{}_field", name).TO_SHOUTY_SNEK_CASE();
    (name, Ident::new(&*field_name, span))
}
