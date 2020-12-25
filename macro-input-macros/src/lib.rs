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
/// automatically derive `From<&[syn::Attribute]>` and `fn strip(attrs: &mut Vec<syn::Attribute>)`
/// # Example
/// ```
/// # use macro_input_macros::MacroInput;
/// #[derive(MacroInput)]
/// pub struct Input {
///     pub flag: Option<()>,
///     pub optional: Option<i32>,
///     #[macro_input(default_value = 3)]
///     pub with_default: i32,
///     pub required: i32,
/// }
///
/// #[derive(MacroInput)]
/// #[macro_input(rename = "YetAnotherName")]
/// pub struct OtherInput {
///     #[macro_input(rename = "new_name")]
///     pub renamed: i32,
/// }
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
