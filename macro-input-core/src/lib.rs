//!
#![deny(missing_docs, clippy::doc_markdown)]

mod convert;
mod field;
mod fields;
#[cfg(feature = "legacy")]
mod r#struct;
mod ty;
mod value;

pub use convert::*;
pub use field::Def;
pub use fields::Defs;
#[cfg(feature = "legacy")]
pub use r#struct::StructLint;
pub use ty::{Type, Types};
pub use value::DefaultValue;
