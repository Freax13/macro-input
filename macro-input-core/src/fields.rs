use crate::FieldDef;
use macro_compose::{Collector, Lint};
use quote::ToTokens;
use syn::{Attribute, Error, Meta, NestedMeta, Path};

/// `FieldDefs` is a collection of [`FieldDef`]s
/// # Example
/// ```
/// use macro_input::{DefaultValue, FieldDef, FieldDefs};
///
/// const BAR_FIELD: FieldDef = FieldDef::new("foo", "bar", false, DefaultValue::Bool(None));
/// const BAZ_FIELD: FieldDef = FieldDef::new("foo", "baz", false, DefaultValue::Str(None));
/// const FOO_FIELDS: &[&FieldDef] = &[&BAR_FIELD, &BAZ_FIELD];
/// const FOO_FIELD_DEFS: FieldDefs = FieldDefs::new(FOO_FIELDS);
/// ```
pub struct FieldDefs<'a> {
    defs: &'a [&'a FieldDef<'a>],
}

impl<'a> FieldDefs<'a> {
    /// create a new collection of [`FieldDef`]s from a slice
    pub const fn new(defs: &'a [&'a FieldDef<'a>]) -> Self {
        FieldDefs { defs }
    }

    /// return an empty collection of [`FieldDef`]s
    pub const fn empty() -> &'static FieldDefs<'static> {
        const EMPTY: FieldDefs<'static> = FieldDefs { defs: &[] };
        &EMPTY
    }

    /// strip the attributes for all field away
    pub fn strip(&self, attrs: &mut Vec<Attribute>) {
        for def in self.defs {
            def.strip(attrs);
        }
    }

    fn has_path(&self, path: &Path) -> bool {
        for def in self.defs.iter() {
            if path.is_ident(def.path) {
                return true;
            }
        }

        false
    }
}

impl<'a> From<&'a [&'a FieldDef<'a>]> for FieldDefs<'a> {
    fn from(defs: &'a [&'a FieldDef<'a>]) -> Self {
        FieldDefs::new(defs)
    }
}

impl Lint<Vec<Attribute>> for FieldDefs<'_> {
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

                                    let is_part_of_defs = |def: &&FieldDef| {
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
