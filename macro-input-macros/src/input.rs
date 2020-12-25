use macro_input_core::{Def, DefaultValue, Defs, StructLint};

pub const RENAME_FIELD: Def = Def::new("macro_input", "rename", false, DefaultValue::Str(None));
pub const DEFAULT_VALUE_FIELD: Def = Def::new(
    "macro_input",
    "default_value",
    false,
    DefaultValue::Any(None),
);

const FIELDS_FIELDS: &[&Def] = &[&RENAME_FIELD, &DEFAULT_VALUE_FIELD];
const FIELDS_FIELD_DEFS: Defs = Defs::new(FIELDS_FIELDS);

const STRUCT_FIELDS: &[&Def] = &[&RENAME_FIELD];
const STRUCT_FIELD_DEFS: Defs = Defs::new(STRUCT_FIELDS);

pub const STRUCT_LINT: StructLint = StructLint::new(&STRUCT_FIELD_DEFS, &FIELDS_FIELD_DEFS);
