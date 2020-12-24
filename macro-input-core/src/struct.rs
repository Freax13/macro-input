use crate::FieldDefs;
use macro_compose::{Collector, Context, Lint};
use syn::{Data, DeriveInput, Error, ItemStruct};

/// a lint for [`syn::ItemStruct`] or [`syn::DeriveInput`]
/// # Example
/// ```
/// use macro_input::{DefaultValue, FieldDef, FieldDefs, StructLint};
///
/// const BAR_FIELD: FieldDef = FieldDef::new("foo", "bar", false, DefaultValue::Bool(None));
/// const BAZ_FIELD: FieldDef = FieldDef::new("foo", "baz", false, DefaultValue::Str(None));
/// const FOO_FIELDS: &[&FieldDef] = &[&BAR_FIELD, &BAZ_FIELD];
/// const FOO_FIELD_DEFS: FieldDefs = FieldDefs::new(FOO_FIELDS);
///
/// const FOO_LINT: StructLint = StructLint::new(FieldDefs::empty(), &FOO_FIELD_DEFS);
/// ```
pub struct StructLint<'a> {
    struct_defs: &'a FieldDefs<'a>,
    fields_defs: &'a FieldDefs<'a>,
}

impl<'a> StructLint<'a> {
    /// create a new struct lint
    ///
    /// `struct_defs` lints attribute on the structs itself and `fields_defs` lints the fields of the struct
    pub const fn new(struct_defs: &'a FieldDefs<'a>, fields_defs: &'a FieldDefs<'a>) -> Self {
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