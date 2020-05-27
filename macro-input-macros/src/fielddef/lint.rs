use crate::{DEFAULT_VALUE_FIELD, RENAME_FIELD};
use macro_compose::{Collector, Context, Lint};
use proc_macro2::Ident;
use std::convert::TryFrom;
use syn::{parse_str, Data, DeriveInput, Error, Meta};

pub struct FieldTypeLint;

impl Lint<DeriveInput> for FieldTypeLint {
    fn lint(&self, input: &DeriveInput, c: &mut Collector) {
        if let Data::Struct(s) = &input.data {
            for field in s.fields.iter() {
                let ty = macro_input_core::Type::try_from(&field.ty);
                match ty {
                    Ok(mut ty) => {
                        let default_value_attribute = DEFAULT_VALUE_FIELD.get_meta(&field.attrs);

                        if ty.optional {
                            if let Some(attr) = &default_value_attribute {
                                c.error(Error::new_spanned(
                                    attr,
                                    "optional fields can't have a default value",
                                ));
                            }
                        }

                        // defaults are optional
                        ty.optional = true;

                        let default_value = default_value_attribute.and_then(|meta| match meta {
                            Meta::NameValue(mnv) => Some(mnv.lit),
                            _ => None,
                        });
                        let default_value = default_value.as_ref();

                        let mut subcontext = Context::new_by_ref(c, &default_value);
                        subcontext.lint(&ty);
                    }
                    Err(e) => c.error(e),
                }
            }
        }
    }
}

pub struct NameLint;

impl Lint<DeriveInput> for NameLint {
    fn lint(&self, input: &DeriveInput, c: &mut Collector) {
        if let Data::Struct(s) = &input.data {
            for field in s.fields.iter() {
                if let Some(name) = RENAME_FIELD.get_value::<Option<String>>(&field.attrs) {
                    if let Err(e) = parse_str::<Ident>(&name) {
                        let meta = RENAME_FIELD.get_lit(&field.attrs);
                        let e = Error::new_spanned(meta, e);
                        c.error(e);
                    }
                } else if field.ident.is_none() {
                    c.error(Error::new_spanned(
                        field,
                        "add #[macro_input(name = $name)] for fields on unnamed structs",
                    ));
                }
            }
        }
    }
}
