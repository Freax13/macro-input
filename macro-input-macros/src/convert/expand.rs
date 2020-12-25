use crate::{field_name, mod_name};
use macro_compose::{Collector, Expand};
use syn::{parse_quote, Data, DeriveInput, Expr, FieldValue, Fields, ItemImpl};

pub struct TryFromAttributes;

impl Expand<DeriveInput> for TryFromAttributes {
    type Output = ItemImpl;

    fn expand(&self, input: &DeriveInput, _: &mut Collector) -> Option<Self::Output> {
        let ident = &input.ident;

        let fields = match &input.data {
            Data::Struct(s) => &s.fields,
            _ => unreachable!(),
        };

        let mod_name = mod_name(input);
        let values = fields.iter().map(|f| -> Expr {
            let (_, ident) = field_name(f);
            parse_quote!(#mod_name::#ident.get_value::<>(attrs)?)
        });

        let block: Expr = match fields {
            Fields::Named(named) => {
                let values = values
                    .zip(
                        named
                            .named
                            .iter()
                            .map(|f| f.ident.as_ref().cloned().unwrap()),
                    )
                    .map(|(value, ident)| -> FieldValue { parse_quote!(#ident: #value) });

                parse_quote!(
                    Self {
                        #(#values),*
                    }
                )
            }
            Fields::Unnamed(_) => parse_quote!(Self (#(#values),*)),
            Fields::Unit => parse_quote!(Self),
        };

        Some(parse_quote!(
            impl ::core::convert::TryFrom<&[::syn::Attribute]> for #ident {
                type Error = ::syn::Error;

                fn try_from(attrs: &[::syn::Attribute]) -> ::syn::Result<Self> {
                    ::std::result::Result::Ok(#block)
                }
            }
        ))
    }
}
