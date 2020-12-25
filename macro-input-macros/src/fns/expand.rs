use crate::mod_name;
use macro_compose::{Collector, Expand};
use syn::{parse_quote, DeriveInput, ItemImpl};

pub struct StripExpand;

impl Expand<DeriveInput> for StripExpand {
    type Output = ItemImpl;

    fn expand(&self, input: &DeriveInput, _: &mut Collector) -> Option<Self::Output> {
        let ident = &input.ident;
        let mod_name = mod_name(input);

        Some(parse_quote!(
            impl #ident {
                /// strip the fields from the attributes
                pub fn strip(attrs: &mut ::std::vec::Vec<::syn::Attribute>) {
                    #mod_name :: FIELD_DEFS .strip(attrs);
                }
            }
        ))
    }
}
