use macro_input_core::{DefaultValue, FieldDef, FieldDefs, StructLint};

pub const RENAME_FIELD: FieldDef =
    FieldDef::new("macro_input", "rename", false, DefaultValue::Str(None));
pub const DEFAULT_VALUE_FIELD: FieldDef = FieldDef::new(
    "macro_input",
    "default_value",
    false,
    DefaultValue::Any(None),
);

const FIELDS_FIELDS: &[&FieldDef] = &[&RENAME_FIELD, &DEFAULT_VALUE_FIELD];
const FIELDS_FIELD_DEFS: FieldDefs = FieldDefs::new(FIELDS_FIELDS);

const STRUCT_FIELDS: &[&FieldDef] = &[&RENAME_FIELD];
const STRUCT_FIELD_DEFS: FieldDefs = FieldDefs::new(STRUCT_FIELDS);

pub const STRUCT_LINT: StructLint = StructLint::new(&STRUCT_FIELD_DEFS, &FIELDS_FIELD_DEFS);
