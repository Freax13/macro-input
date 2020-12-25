use crate::Defs;
use macro_compose::{Collector, Context, Lint};
use syn::{Data, DeriveInput, Error, ItemStruct};

/// a lint for [`syn::ItemStruct`] or [`syn::DeriveInput`]
/// # Example
/// ```
/// # use macro_input_core as macro_input;
/// use macro_input::{DefaultValue, Def, Defs, StructLint};
///
/// const BAR_FIELD: Def = Def::new("foo", "bar", false, DefaultValue::Bool(None));
/// const BAZ_FIELD: Def = Def::new("foo", "baz", false, DefaultValue::Str(None));
/// const FOO_FIELDS: &[&Def] = &[&BAR_FIELD, &BAZ_FIELD];
/// const FOO_FIELD_DEFS: Defs = Defs::new(FOO_FIELDS);
///
/// const FOO_LINT: StructLint = StructLint::new(Defs::empty(), &FOO_FIELD_DEFS);
/// ```
pub struct StructLint<'a> {
    struct_defs: &'a Defs<'a>,
    fields_defs: &'a Defs<'a>,
}

impl<'a> StructLint<'a> {
    /// create a new struct lint
    ///
    /// `struct_defs` lints attribute on the structs itself and `fields_defs` lints the fields of the struct
    #[must_use]
    pub const fn new(struct_defs: &'a Defs<'a>, fields_defs: &'a Defs<'a>) -> Self {
        StructLint {
            struct_defs,
            fields_defs,
        }
    }
}

impl Lint<ItemStruct> for StructLint<'_> {
    fn lint(&self, input: &ItemStruct, c: &mut Collector) {
        let derive_input = DeriveInput::from(input.clone());
        let mut subcontext = Context::new_by_ref(c, &derive_input);
        subcontext.lint(self);
    }
}

impl Lint<DeriveInput> for StructLint<'_> {
    fn lint(&self, input: &DeriveInput, c: &mut Collector) {
        let mut subcontext = Context::new_by_ref(c, &input.attrs);
        subcontext.lint(self.struct_defs);

        match &input.data {
            Data::Struct(s) => {
                for field in s.fields.iter() {
                    let mut subcontext = Context::new_by_ref(c, &field.attrs);
                    subcontext.lint(self.fields_defs);
                }
            }
            _ => c.error(Error::new_spanned(input, "expected a struct")),
        }
    }
}
