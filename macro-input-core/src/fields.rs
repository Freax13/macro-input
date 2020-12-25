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
/// use macro_input::{DefaultValue, Def, Defs};
///
/// const BAR_FIELD: Def = Def::new("foo", "bar", false, DefaultValue::Bool(None));
/// const BAZ_FIELD: Def = Def::new("foo", "baz", false, DefaultValue::Str(None));
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
    ///
    /// This is useful for attribute macros because rust has no way of knowing which attributes were used.
    /// ```
    /// # use macro_input_core as macro_input;
    /// use macro_input::{DefaultValue, Def, Defs};
    /// use syn::{parse_quote, Attribute};
    ///
    /// // construct some attributes
    /// let attr1: Attribute = parse_quote!(#[foo(bar = false)]);
    /// let attr2: Attribute = parse_quote!(#[foo(baz = "baz")]);
    /// let attr3: Attribute = parse_quote!(#[foo(qux = "qux", quux = 3)]);
    /// let attr4: Attribute = parse_quote!(#[foo(quuz = 'b')]);
    /// let attr5: Attribute = parse_quote!(#[some(thing = "value")]);
    /// let mut attrs = vec![attr1, attr2, attr3, attr4, attr5];
    ///
    /// // strip away all mentions of the fields bar, baz and qux in foo
    /// const BAR_FIELD: Def = Def::new("foo", "bar", false, DefaultValue::Bool(None));
    /// const BAZ_FIELD: Def = Def::new("foo", "baz", false, DefaultValue::Str(None));
    /// const QUX_FIELD: Def = Def::new("foo", "qux", false, DefaultValue::Str(None));
    /// const FOO_FIELDS: &[&Def] = &[&BAR_FIELD, &BAZ_FIELD, &QUX_FIELD];
    /// const FOO_FIELD_DEFS: Defs = Defs::new(FOO_FIELDS);
    /// FOO_FIELD_DEFS.strip(&mut attrs);
    ///
    /// // the Vec no longer contains
    /// // #[foo(bar = false)], #[foo(baz = "baz")] and #[foo(qux = "qux")]
    /// // but
    /// // #[foo(quux = 3)], #[foo(quuz = 'b')], #[some(thing = "value")]
    /// // remain
    /// let attr1: Attribute = parse_quote!(#[foo(quux = 3)]);
    /// let attr2: Attribute = parse_quote!(#[foo(quuz = 'b')]);
    /// let attr3: Attribute = parse_quote!(#[some(thing = "value")]);
    /// assert_eq!(attrs, vec![attr1, attr2, attr3]);
    /// ```
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
