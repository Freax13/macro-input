use crate::{Type, Types};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_quote, Error, Lit, LitByteStr, Result};

#[derive(Clone)]
/// a default value for a [`Def`](crate::Def)
pub enum Default {
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

impl Default {
    /// construct a `Default` from a type and a literal
    /// # Errors
    /// may return an error if the literal is not compatible with the type
    pub fn from_lit(ty: Type, lit: Option<Lit>) -> Result<Default> {
        match ty.ty {
            Types::Any => Ok(Default::Any(lit)),
            Types::Flag => Ok(Default::Flag),
            Types::Str => lit
                .map(|lit| {
                    if let Lit::Str(v) = lit {
                        Ok(v.value())
                    } else {
                        Err(Error::new(lit.span(), "expected string"))
                    }
                })
                .transpose()
                .map(Default::String),
            Types::ByteStr => lit
                .map(|lit| {
                    if let Lit::ByteStr(v) = lit {
                        Ok(v.value())
                    } else {
                        Err(Error::new(lit.span(), "expected bytes"))
                    }
                })
                .transpose()
                .map(Default::ByteString),
            Types::Byte => lit
                .map(|lit| {
                    if let Lit::Byte(v) = lit {
                        Ok(v.value())
                    } else {
                        Err(Error::new(lit.span(), "expected byte"))
                    }
                })
                .transpose()
                .map(Default::Byte),
            Types::Char => lit
                .map(|lit| {
                    if let Lit::Char(v) = lit {
                        Ok(v.value())
                    } else {
                        Err(Error::new(lit.span(), "expected char"))
                    }
                })
                .transpose()
                .map(Default::Char),
            Types::I32 => lit
                .map(|lit| {
                    if let Lit::Int(v) = lit {
                        v.base10_parse()
                    } else {
                        Err(Error::new(lit.span(), "expected i32"))
                    }
                })
                .transpose()
                .map(Default::I32),
            Types::F32 => lit
                .map(|lit| {
                    if let Lit::Int(v) = lit {
                        v.base10_parse()
                    } else {
                        Err(Error::new(lit.span(), "expected f32"))
                    }
                })
                .transpose()
                .map(Default::F32),
            Types::Bool => lit
                .map(|lit| {
                    if let Lit::Bool(v) = lit {
                        Ok(v.value)
                    } else {
                        Err(Error::new(lit.span(), "expected bool"))
                    }
                })
                .transpose()
                .map(Default::Bool),
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
            Default::Any(val) => val.is_some(),
            Default::Flag => false,
            Default::Str(val) => val.is_some(),
            Default::String(val) => val.is_some(),
            Default::ByteStr(val) => val.is_some(),
            Default::ByteString(val) => val.is_some(),
            Default::Byte(val) => val.is_some(),
            Default::Char(val) => val.is_some(),
            Default::I32(val) => val.is_some(),
            Default::F32(val) => val.is_some(),
            Default::Bool(val) => val.is_some(),
        }
    }

    pub(crate) fn as_lit(&self) -> Option<Lit> {
        match self {
            Default::Flag => None,
            Default::Any(val) => val.clone(),
            Default::Str(val) => val.map(|v| parse_quote!(#v)),
            Default::String(val) => val.as_ref().map(|v| parse_quote!(#v)),
            Default::ByteStr(val) => val.map(|v| {
                let lbs = LitByteStr::new(&v, Span::call_site());
                parse_quote!(#lbs)
            }),
            Default::ByteString(val) => val.as_ref().map(|v| {
                let lbs = LitByteStr::new(&v, Span::call_site());
                parse_quote!(#lbs)
            }),
            Default::Byte(val) => val.map(|v| parse_quote!(#v)),
            Default::Char(val) => val.map(|v| parse_quote!(#v)),
            Default::I32(val) => val.map(|v| parse_quote!(#v)),
            Default::F32(val) => val.map(|v| parse_quote!(#v)),
            Default::Bool(val) => val.map(|v| parse_quote!(#v)),
        }
    }
}

impl From<Default> for Option<Lit> {
    fn from(val: Default) -> Self {
        match val {
            Default::Flag => None,
            Default::Any(val) => val,
            Default::Str(val) => val.map(|v| parse_quote!(#v)),
            Default::String(val) => val.map(|v| parse_quote!(#v)),
            Default::ByteStr(val) => val.map(|v| {
                let lbs = LitByteStr::new(&v, Span::call_site());
                parse_quote!(#lbs)
            }),
            Default::ByteString(val) => val.map(|v| {
                let lbs = LitByteStr::new(&v, Span::call_site());
                parse_quote!(#lbs)
            }),
            Default::Byte(val) => val.map(|v| parse_quote!(#v)),
            Default::Char(val) => val.map(|v| parse_quote!(#v)),
            Default::I32(val) => val.map(|v| parse_quote!(#v)),
            Default::F32(val) => val.map(|v| parse_quote!(#v)),
            Default::Bool(val) => val.map(|v| parse_quote!(#v)),
        }
    }
}

impl From<&Default> for Types {
    fn from(value: &Default) -> Self {
        match value {
            Default::Any(_) => Types::Any,
            Default::Flag => Types::Flag,
            Default::Str(_) | Default::String(_) => Types::Str,
            Default::ByteStr(_) | Default::ByteString(_) => Types::ByteStr,
            Default::Byte(_) => Types::Byte,
            Default::Char(_) => Types::Char,
            Default::I32(_) => Types::I32,
            Default::F32(_) => Types::F32,
            Default::Bool(_) => Types::Bool,
        }
    }
}

impl ToTokens for Default {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        fn map_literal<V: ToTokens>(v: &Option<V>) -> TokenStream {
            if let Some(data) = v {
                quote!(::core::option::Option::Some(#data))
            } else {
                quote!(::core::option::Option::None)
            }
        }

        let tts = match self {
            Default::Any(v) => {
                let data = map_literal(v);
                quote!(::macro_input::Default::Any(#data))
            }
            Default::Flag => quote!(::macro_input::Default::Flag),
            Default::Str(v) => {
                let data = map_literal(v);
                quote!(::macro_input::Default::Str(#data))
            }
            Default::String(v) => {
                let data = map_literal(v);
                quote!(::macro_input::Default::Str(#data))
            }
            Default::ByteStr(v) => {
                let data = map_literal(&v.as_ref().map(|v| LitByteStr::new(v, Span::call_site())));
                quote!(::macro_input::Default::ByteStr(#data))
            }
            Default::ByteString(v) => {
                let data = map_literal(&v.as_ref().map(|v| LitByteStr::new(v, Span::call_site())));
                quote!(::macro_input::Default::ByteStr(#data))
            }
            Default::Byte(v) => {
                let data = map_literal(v);
                quote!(::macro_input::Default::Byte(#data))
            }
            Default::Char(v) => {
                let data = map_literal(v);
                quote!(::macro_input::Default::Char(#data))
            }
            Default::I32(v) => {
                let data = map_literal(v);
                quote!(::macro_input::Default::I32(#data))
            }
            Default::F32(v) => {
                let data = map_literal(v);
                quote!(::macro_input::Default::F32(#data))
            }
            Default::Bool(v) => {
                let data = map_literal(v);
                quote!(::macro_input::Default::Bool(#data))
            }
        };
        tokens.extend(tts);
    }
}
