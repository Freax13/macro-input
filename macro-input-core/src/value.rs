use crate::{Type, Types};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_quote, Error, Lit, LitByteStr, Result};

#[derive(Clone)]
/// a default value for a [`Def`](crate::Def)
pub enum DefaultValue {
    /// any literal
    Any(Option<Lit>),
    /// a flag that doesn't have a value eg #[my_input(enabled)]
    Flag,
    /// a string
    Str(Option<&'static str>),
    /// a string
    String(Option<String>),
    /// a bytestring
    ByteStr(Option<&'static [u8]>),
    /// a bytestring
    ByteString(Option<Vec<u8>>),
    /// a u8
    Byte(Option<u8>),
    /// a char
    Char(Option<char>),
    /// a i32
    I32(Option<i32>),
    /// a f32
    F32(Option<f32>),
    /// a bool
    Bool(Option<bool>),
}

impl DefaultValue {
    /// construct a `DefaultValue` from a type and a literal
    /// # Errors
    /// may return an error if the literal is not compatible with the type
    pub fn from_lit(ty: Type, lit: Option<Lit>) -> Result<DefaultValue> {
        match ty.ty {
            Types::Any => Ok(DefaultValue::Any(lit)),
            Types::Flag => Ok(DefaultValue::Flag),
            Types::Str => lit
                .map(|lit| {
                    if let Lit::Str(v) = lit {
                        Ok(v.value())
                    } else {
                        Err(Error::new(lit.span(), "expected string"))
                    }
                })
                .transpose()
                .map(DefaultValue::String),
            Types::ByteStr => lit
                .map(|lit| {
                    if let Lit::ByteStr(v) = lit {
                        Ok(v.value())
                    } else {
                        Err(Error::new(lit.span(), "expected bytes"))
                    }
                })
                .transpose()
                .map(DefaultValue::ByteString),
            Types::Byte => lit
                .map(|lit| {
                    if let Lit::Byte(v) = lit {
                        Ok(v.value())
                    } else {
                        Err(Error::new(lit.span(), "expected byte"))
                    }
                })
                .transpose()
                .map(DefaultValue::Byte),
            Types::Char => lit
                .map(|lit| {
                    if let Lit::Char(v) = lit {
                        Ok(v.value())
                    } else {
                        Err(Error::new(lit.span(), "expected char"))
                    }
                })
                .transpose()
                .map(DefaultValue::Char),
            Types::I32 => lit
                .map(|lit| {
                    if let Lit::Int(v) = lit {
                        v.base10_parse()
                    } else {
                        Err(Error::new(lit.span(), "expected i32"))
                    }
                })
                .transpose()
                .map(DefaultValue::I32),
            Types::F32 => lit
                .map(|lit| {
                    if let Lit::Int(v) = lit {
                        v.base10_parse()
                    } else {
                        Err(Error::new(lit.span(), "expected f32"))
                    }
                })
                .transpose()
                .map(DefaultValue::F32),
            Types::Bool => lit
                .map(|lit| {
                    if let Lit::Bool(v) = lit {
                        Ok(v.value)
                    } else {
                        Err(Error::new(lit.span(), "expected bool"))
                    }
                })
                .transpose()
                .map(DefaultValue::Bool),
        }
    }

    /// get the type of the value
    #[must_use]
    pub fn ty(&self, optional: bool) -> Type {
        Type {
            ty: Types::from(self),
            optional,
        }
    }

    /// checks whether the value is a default value
    #[must_use]
    pub fn has_default_data(&self) -> bool {
        match self {
            DefaultValue::Any(val) => val.is_some(),
            DefaultValue::Flag => false,
            DefaultValue::Str(val) => val.is_some(),
            DefaultValue::String(val) => val.is_some(),
            DefaultValue::ByteStr(val) => val.is_some(),
            DefaultValue::ByteString(val) => val.is_some(),
            DefaultValue::Byte(val) => val.is_some(),
            DefaultValue::Char(val) => val.is_some(),
            DefaultValue::I32(val) => val.is_some(),
            DefaultValue::F32(val) => val.is_some(),
            DefaultValue::Bool(val) => val.is_some(),
        }
    }

    pub(crate) fn as_lit(&self) -> Option<Lit> {
        match self {
            DefaultValue::Flag => None,
            DefaultValue::Any(val) => val.clone(),
            DefaultValue::Str(val) => val.map(|v| parse_quote!(#v)),
            DefaultValue::String(val) => val.as_ref().map(|v| parse_quote!(#v)),
            DefaultValue::ByteStr(val) => val.map(|v| {
                let lbs = LitByteStr::new(&v, Span::call_site());
                parse_quote!(#lbs)
            }),
            DefaultValue::ByteString(val) => val.as_ref().map(|v| {
                let lbs = LitByteStr::new(&v, Span::call_site());
                parse_quote!(#lbs)
            }),
            DefaultValue::Byte(val) => val.map(|v| parse_quote!(#v)),
            DefaultValue::Char(val) => val.map(|v| parse_quote!(#v)),
            DefaultValue::I32(val) => val.map(|v| parse_quote!(#v)),
            DefaultValue::F32(val) => val.map(|v| parse_quote!(#v)),
            DefaultValue::Bool(val) => val.map(|v| parse_quote!(#v)),
        }
    }
}

impl From<DefaultValue> for Option<Lit> {
    fn from(val: DefaultValue) -> Self {
        match val {
            DefaultValue::Flag => None,
            DefaultValue::Any(val) => val,
            DefaultValue::Str(val) => val.map(|v| parse_quote!(#v)),
            DefaultValue::String(val) => val.map(|v| parse_quote!(#v)),
            DefaultValue::ByteStr(val) => val.map(|v| {
                let lbs = LitByteStr::new(&v, Span::call_site());
                parse_quote!(#lbs)
            }),
            DefaultValue::ByteString(val) => val.map(|v| {
                let lbs = LitByteStr::new(&v, Span::call_site());
                parse_quote!(#lbs)
            }),
            DefaultValue::Byte(val) => val.map(|v| parse_quote!(#v)),
            DefaultValue::Char(val) => val.map(|v| parse_quote!(#v)),
            DefaultValue::I32(val) => val.map(|v| parse_quote!(#v)),
            DefaultValue::F32(val) => val.map(|v| parse_quote!(#v)),
            DefaultValue::Bool(val) => val.map(|v| parse_quote!(#v)),
        }
    }
}

impl From<&DefaultValue> for Types {
    fn from(value: &DefaultValue) -> Self {
        match value {
            DefaultValue::Any(_) => Types::Any,
            DefaultValue::Flag => Types::Flag,
            DefaultValue::Str(_) | DefaultValue::String(_) => Types::Str,
            DefaultValue::ByteStr(_) | DefaultValue::ByteString(_) => Types::ByteStr,
            DefaultValue::Byte(_) => Types::Byte,
            DefaultValue::Char(_) => Types::Char,
            DefaultValue::I32(_) => Types::I32,
            DefaultValue::F32(_) => Types::F32,
            DefaultValue::Bool(_) => Types::Bool,
        }
    }
}

impl ToTokens for DefaultValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        fn map_literal<V: ToTokens>(v: &Option<V>) -> TokenStream {
            if let Some(data) = v {
                quote!(::core::option::Option::Some(#data))
            } else {
                quote!(::core::option::Option::None)
            }
        }

        let tts = match self {
            DefaultValue::Any(v) => {
                let data = map_literal(v);
                quote!(::macro_input::DefaultValue::Any(#data))
            }
            DefaultValue::Flag => quote!(::macro_input::DefaultValue::Flag),
            DefaultValue::Str(v) => {
                let data = map_literal(v);
                quote!(::macro_input::DefaultValue::Str(#data))
            }
            DefaultValue::String(v) => {
                let data = map_literal(v);
                quote!(::macro_input::DefaultValue::Str(#data))
            }
            DefaultValue::ByteStr(v) => {
                let data = map_literal(&v.as_ref().map(|v| LitByteStr::new(v, Span::call_site())));
                quote!(::macro_input::DefaultValue::ByteStr(#data))
            }
            DefaultValue::ByteString(v) => {
                let data = map_literal(&v.as_ref().map(|v| LitByteStr::new(v, Span::call_site())));
                quote!(::macro_input::DefaultValue::ByteStr(#data))
            }
            DefaultValue::Byte(v) => {
                let data = map_literal(v);
                quote!(::macro_input::DefaultValue::Byte(#data))
            }
            DefaultValue::Char(v) => {
                let data = map_literal(v);
                quote!(::macro_input::DefaultValue::Char(#data))
            }
            DefaultValue::I32(v) => {
                let data = map_literal(v);
                quote!(::macro_input::DefaultValue::I32(#data))
            }
            DefaultValue::F32(v) => {
                let data = map_literal(v);
                quote!(::macro_input::DefaultValue::F32(#data))
            }
            DefaultValue::Bool(v) => {
                let data = map_literal(v);
                quote!(::macro_input::DefaultValue::Bool(#data))
            }
        };
        tokens.extend(tts);
    }
}
