use crate::RENAME_FIELD;
use macro_compose::{Collector, Lint};
use proc_macro2::Ident;
use syn::{parse_str, DeriveInput, Error};

pub struct Name;

impl Lint<DeriveInput> for Name {
    fn lint(&self, input: &DeriveInput, c: &mut Collector) {
        if let Some(name) = RENAME_FIELD
            .get_value::<Option<String>>(&input.attrs)
            .unwrap()
        {
            if let Err(e) = parse_str::<Ident>(&name) {
                let meta = RENAME_FIELD.get_lit(&input.attrs).unwrap();
                let e = Error::new_spanned(meta, e);
                c.error(e);
            }
        }
    }
}
