use crate::Def;
#[cfg(feature = "legacy")]
use macro_compose::{Collector, Lint};
#[cfg(feature = "legacy")]
use quote::ToTokens;
use syn::Attribute;
#[cfg(feature = "legacy")]
use syn::{Error, Meta, NestedMeta, Path};

/// `Defs` is a collection of [`Def`]s
/// # Example
/// ```
/// # use macro_input_core as macro_input;
/// use macro_input::{Default, Def, Defs};
///
/// const BAR_FIELD: Def = Def::new("foo", "bar", false, Default::Bool(None));
/// const BAZ_FIELD: Def = Def::new("foo", "baz", false, Default::Str(None));
/// const FOO_FIELDS: &[&Def] = &[&BAR_FIELD, &BAZ_FIELD];
/// const FOO_FIELD_DEFS: Defs = Defs::new(FOO_FIELDS);
/// ```
pub struct Defs<'a> {
    defs: &'a [&'a Def<'a>],
}

impl<'a> Defs<'a> {
    /// create a new collection of [`Def`]s from a slice
    #[must_use]
    pub const fn new(defs: &'a [&'a Def<'a>]) -> Self {
        Defs { defs }
    }

    /// return an empty collection of [`Def`]s
    #[must_use]
    pub const fn empty() -> &'static Defs<'static> {
        const EMPTY: Defs<'static> = Defs { defs: &[] };
        &EMPTY
    }

    /// strip away the attributes for all fields
    pub fn strip(&self, attrs: &mut Vec<Attribute>) {
        for def in self.defs {
            def.strip(attrs);
        }
    }

    #[cfg(feature = "legacy")]
    fn has_path(&self, path: &Path) -> bool {
        for def in self.defs.iter() {
            if path.is_ident(def.path) {
                return true;
            }
        }

        false
    }
}

impl<'a> From<&'a [&'a Def<'a>]> for Defs<'a> {
    fn from(defs: &'a [&'a Def<'a>]) -> Self {
        Defs::new(defs)
    }
}

#[cfg(feature = "legacy")]
impl Lint<Vec<Attribute>> for Defs<'_> {
    fn lint(&self, input: &Vec<Attribute>, c: &mut Collector) {
        for def in self.defs.iter() {
            def.lint(input, c);
        }

        for attr in input.iter() {
            let meta = attr.parse_meta().unwrap();
            let path = meta.path();
            if self.has_path(path) {
                match &meta {
                    Meta::List(list) => {
                        for meta in list.nested.iter() {
                            match meta {
                                NestedMeta::Meta(meta) => {
                                    match meta {
                                        Meta::NameValue(_) | Meta::Path(_) => {}
                                        _ => {
                                            c.error(Error::new_spanned(
                                                meta,
                                                "expected name-and-value or path meta",
                                            ));
                                        }
                                    }

                                    let is_part_of_defs = |def: &&Def| {
                                        path.is_ident(&def.path) && meta.path().is_ident(&def.name)
                                    };
                                    if !self.defs.iter().any(is_part_of_defs) {
                                        c.error(Error::new_spanned(
                                            meta,
                                            format!(
                                                "unrecognized attribute: {}::{}",
                                                path.to_token_stream(),
                                                meta.path().to_token_stream()
                                            ),
                                        ));
                                    }
                                }
                                NestedMeta::Lit(l) => {
                                    c.error(Error::new_spanned(l, "expected meta"));
                                }
                            }
                        }
                    }
                    _ => c.error(Error::new_spanned(meta, "expected a list meta")),
                }
            }
        }
    }
}
