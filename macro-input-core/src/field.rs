use crate::{convert::FromMeta, DefaultValue};
#[cfg(feature = "legacy")]
use macro_compose::{Collector, Context, Lint};
use proc_macro2::Span;
use quote::{format_ident, ToTokens};
use std::{iter::FromIterator, mem::replace};
use syn::{
    parse::Parse, parse2, parse_quote, punctuated::Punctuated, Attribute, Error, Lit, Meta,
    NestedMeta, Result,
};

/// a field definition
/// # Example
/// ```
/// # use macro_input_core as macro_input;
/// use macro_input::{DefaultValue, Def};
///
/// const BAR_FIELD: Def = Def::new("foo", "bar", false, DefaultValue::Bool(None));
/// const BAZ_FIELD: Def = Def::new("foo", "baz", false, DefaultValue::Str(None));
/// ```
pub struct Def<'a> {
    /// the path/namespace of the field
    pub path: &'a str,
    /// the name of the field
    pub name: &'a str,
    /// whether or not this field is required
    pub required: bool,
    /// the typed default value
    pub default: DefaultValue,
}

impl<'a> Def<'a> {
    /// create a new field definition
    #[must_use]
    pub const fn new(path: &'a str, name: &'a str, required: bool, default: DefaultValue) -> Self {
        Def {
            path,
            name,
            required,
            default,
        }
    }

    /// strip away the attributes for this field
    ///
    /// This is useful for attribute macros because rust has no way of knowing which attributes were used.
    /// ```
    /// # use macro_input_core as macro_input;
    /// use macro_input::{DefaultValue, Def};
    /// use syn::{parse_quote, Attribute};
    ///
    /// // construct some attributes
    /// let foo_attr: Attribute = parse_quote!(#[foo(bar = false)]);
    /// let other_attr: Attribute = parse_quote!(#[some(thing = "value")]);
    /// let mut attrs = vec![foo_attr, other_attr.clone()];
    ///
    /// // strip away all mentions of the field bar in foo
    /// const BAR_FIELD: Def = Def::new("foo", "bar", false, DefaultValue::Bool(None));
    /// BAR_FIELD.strip(&mut attrs);
    ///
    /// // the Vec no longer contains #[foo(bar = false)] but #[some(thing = "value")] remains
    /// assert_eq!(attrs, vec![other_attr]);
    /// ```
    pub fn strip(&self, attrs: &mut Vec<Attribute>) {
        let data = replace(attrs, Vec::new());
        attrs.extend(data.into_iter().filter_map(|mut a| {
            if self.strip_from_attribute(&mut a) {
                None
            } else {
                Some(a)
            }
        }));
    }

    /// strip the attribute and return whether it was empty
    fn strip_from_attribute(&self, attr: &mut Attribute) -> bool {
        let mut meta = if let Ok(meta) = attr.parse_meta() {
            meta
        } else {
            return false;
        };

        // check the path
        if !meta.path().is_ident(self.path) {
            return false;
        }

        match &mut meta {
            Meta::List(list) => {
                let new_punctuated = list
                    .nested
                    .iter()
                    .filter(|meta| {
                        if let NestedMeta::Meta(meta) = meta {
                            !meta.path().is_ident(self.name)
                        } else {
                            true
                        }
                    })
                    .cloned();
                list.nested = Punctuated::from_iter(new_punctuated);

                let empty = list.nested.is_empty();
                *attr = parse_quote!(#[#list]);
                empty
            }
            Meta::Path(_) => true,
            _ => false,
        }
    }

    /// try to find the meta that has the value for this field
    ///
    /// # Errors
    /// may return the error if the field is required but not found
    pub fn get_meta(&self, attrs: &[Attribute]) -> Result<Option<Meta>> {
        for attr in attrs.iter() {
            let meta = attr.parse_meta()?;
            if meta.path().is_ident(self.path) {
                if let Meta::List(list) = meta {
                    for meta in list.nested.iter() {
                        if let NestedMeta::Meta(meta) = meta {
                            if meta.path().is_ident(self.name) {
                                return Ok(Some(meta.clone()));
                            }
                        }
                    }
                }
            }
        }

        if self.required {
            return Err(Error::new(
                Span::call_site(),
                format!(
                    "attribute for required field not found: {}::{}",
                    self.path, self.name
                ),
            ));
        }

        // construct a default meta
        if let Some(lit) = self.default.as_lit() {
            let name = format_ident!("{}", self.path);

            return Ok(Some(parse_quote!(#name = #lit)));
        }

        Ok(None)
    }

    /// try to find the literal that has the value for this field
    ///
    /// # Errors
    /// may return the error if the field is required but not found
    pub fn get_lit(&self, attrs: &[Attribute]) -> Result<Option<Lit>> {
        Ok(self.get_meta(attrs)?.and_then(|m| match m {
            Meta::NameValue(nvm) => Some(nvm.lit),
            _ => None,
        }))
    }

    /// try to parse the literal that has the value for this field
    ///
    /// # Errors
    /// may return the error if parsing fails
    pub fn get<L: Parse>(&self, attrs: &[Attribute]) -> Result<Option<L>> {
        self.get_lit(attrs)?
            .map(|lit| {
                let tokens = lit.to_token_stream();
                parse2(tokens)
            })
            .transpose()
    }

    /// try to extract the value from the literal that has the value for this field
    ///
    /// ```
    /// # use macro_input_core as macro_input;
    /// use macro_input::{DefaultValue, Def};
    /// use syn::{parse_quote, Attribute};
    ///
    /// # fn main() -> syn::Result<()> {
    /// let attr = parse_quote!(#[foo(bar = false)]);
    /// const BAR_FIELD: Def = Def::new("foo", "bar", false, DefaultValue::Bool(None));
    /// let value = BAR_FIELD.get_value::<bool>(&[attr])?;
    /// assert_eq!(value, false);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// may return an error if the field doesn't exist or has a value of the wrong type
    pub fn get_value<V: FromMeta>(&self, attrs: &[Attribute]) -> Result<V> {
        self.get_meta(attrs).and_then(FromMeta::from)
    }
}

#[cfg(feature = "legacy")]
impl Lint<Vec<Attribute>> for Def<'_> {
    fn lint(&self, input: &Vec<Attribute>, c: &mut Collector) {
        let mut found = false;

        for attr in input.iter() {
            let meta = attr.parse_meta().unwrap();
            if meta.path().is_ident(self.path) {
                if let Meta::List(list) = meta {
                    for meta in list.nested.iter() {
                        if let NestedMeta::Meta(meta) = meta {
                            if meta.path().is_ident(self.name) {
                                match meta {
                                    Meta::NameValue(meta) => {
                                        if found {
                                            c.error(Error::new_spanned(
                                                meta,
                                                format!("dupplicate {} attribute", self.path),
                                            ));
                                        } else {
                                            found = true;
                                        }

                                        let some_lit = Some(&meta.lit);
                                        let mut subcontext = Context::new_by_ref(c, &some_lit);
                                        subcontext.lint(
                                            &self
                                                .default
                                                .ty(!self.required
                                                    && !self.default.has_default_data()),
                                        );
                                    }
                                    Meta::Path(_) => {
                                        if found {
                                            c.error(Error::new_spanned(
                                                meta,
                                                format!("dupplicate {} attribute", self.path),
                                            ));
                                        } else {
                                            found = true;
                                        }

                                        let mut subcontext = Context::new_by_ref(c, &None);
                                        subcontext.lint(
                                            &self
                                                .default
                                                .ty(!self.required
                                                    && !self.default.has_default_data()),
                                        );
                                    }
                                    _ => {
                                        c.error(Error::new_spanned(&meta, "unexpected meta list"));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if !found && self.required {
            c.error(Error::new(
                Span::call_site(),
                format!("missing required {} attribute", self.name),
            ));
        }
    }
}
