use crate::{field_name, mod_name, DEFAULT_VALUE_FIELD, RENAME_FIELD};
use heck::{ShoutySnekCase, SnekCase};
use macro_compose::{Collector, Context, Expand};
use macro_input_core::DefaultValue;
use proc_macro2::Ident;
use std::convert::TryFrom;
use syn::{parse_quote, Data, DeriveInput, Expr, Field, ItemConst, ItemMod, LitStr, Type};

pub struct ConstFieldsExpand;

impl Expand<DeriveInput> for ConstFieldsExpand {
    type Output = ItemMod;

    fn expand(&self, input: &DeriveInput, c: &mut Collector) -> Option<Self::Output> {
        let path = RENAME_FIELD
            .get_value::<Option<String>>(&input.attrs)
            .unwrap()
            .unwrap_or_else(|| input.ident.to_string().to_snek_case());

        let const_field_expand = ConstFieldExpand { path };

        let fields = match &input.data {
            Data::Struct(s) => &s.fields,
            _ => unreachable!(),
        };

        let const_fields = fields.iter().map(|f| {
            let mut subcontext = Context::new_by_ref(c, f);
            subcontext.capture(&const_field_expand)
        });

        let field_refs = fields.iter().map(|f| -> Expr {
            let (_, ident) = field_name(f);
            parse_quote!(#ident)
        });

        let mod_ident = mod_name(input);
        Some(parse_quote!(
            mod #mod_ident {
                #(#const_fields)*

                const FIELDS: &[&::macro_input::FieldDef] = &[#(&#field_refs),*];
                pub const FIELD_DEFS: ::macro_input::FieldDefs = ::macro_input::FieldDefs::new(FIELDS);
            }
        ))
    }
}

struct ConstFieldExpand {
    path: String,
}

impl Expand<Field> for ConstFieldExpand {
    type Output = ItemConst;

    fn expand(&self, f: &Field, _: &mut Collector) -> Option<Self::Output> {
        fn field_name(f: &Field) -> (String, Ident) {
            let (name, span) = match RENAME_FIELD.get::<LitStr>(&f.attrs) {
                Some(s) => (s.value(), s.span()),
                None => {
                    let ident = f.ident.as_ref().unwrap();
                    (ident.to_string(), ident.span())
                }
            };
            let field_name = format!("{}_field", name).TO_SHOUTY_SNEK_CASE();
            (name, Ident::new(&*field_name, span))
        }

        fn is_optional(f: &Field) -> bool {
            if let Type::Path(tp) = &f.ty {
                tp.path.segments.len() == 1 && tp.path.segments.first().unwrap().ident == "Option"
            } else {
                false
            }
        }

        let (name, ident) = field_name(f);

        let default_value = DEFAULT_VALUE_FIELD.get_lit(&f.attrs);
        let value = DefaultValue::from_lit(
            macro_input_core::Type::try_from(&f.ty).unwrap(),
            default_value.as_ref(),
        );

        let optional = is_optional(f) || default_value.is_some();
        let required = !optional;

        let path = &self.path;
        Some(parse_quote!(
            pub const #ident: ::macro_input::FieldDef =
            ::macro_input::FieldDef::new(#path, #name, #required, #value);
        ))
    }
}
