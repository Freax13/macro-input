#[cfg(feature = "legacy")]
use macro_compose::{Collector, Lint};
use quote::ToTokens;
use std::convert::TryFrom;
use syn::{parse_quote, Error, GenericArgument, Lit, Path, PathArguments};

#[derive(Clone, Copy, Debug)]
/// the type of a field
pub struct Type {
    /// the actual type
    pub ty: Types,
    /// whether or not the value for the type is optional
    pub optional: bool,
}

#[cfg(feature = "legacy")]
impl<'a> Lint<Option<&'a Lit>> for Type {
    fn lint(&self, input: &Option<&'a Lit>, c: &mut Collector) {
        let ty = match self.ty {
            Types::Any => "anything",
            Types::Flag => "nothing",
            Types::Str => "string",
            Types::ByteStr => "byte string",
            Types::Byte => "byte",
            Types::Char => "char",
            Types::I32 => "i32",
            Types::F32 => "f32",
            Types::Bool => "bool",
        };

        match (input, self.ty) {
            (Some(_), Types::Any) => {}
            (None, Types::Flag) => {}
            (None, _) if self.optional => {}
            (Some(Lit::Str(_)), Types::Str) => {}
            (Some(Lit::ByteStr(_)), Types::ByteStr) => {}
            (Some(Lit::Byte(_)), Types::Byte) => {}
            (Some(Lit::Char(_)), Types::Char) => {}
            (Some(Lit::Int(_)), Types::I32) => {}
            (Some(Lit::Float(_)), Types::F32) => {}
            (Some(Lit::Bool(_)), Types::Bool) => {}
            (Some(lit), _) => c.error(Error::new_spanned(
                input,
                format!("expected {}, got {}", ty, lit.to_token_stream()),
            )),
            (None, _) => c.error(Error::new_spanned(
                input,
                format!("expected {}, got nothing", ty,),
            )),
        }
    }
}
#[derive(Clone, Copy, Debug)]
/// all the types a field can contain
pub enum Types {
    /// can contain any literal
    Any,
    /// doesn't have a value eg #[my_input(enabled)]
    Flag,
    /// for string
    Str,
    /// for bytestring
    ByteStr,
    /// for u8
    Byte,
    /// for char
    Char,
    /// for i32
    I32,
    /// for f32
    F32,
    /// for bool
    Bool,
}

impl TryFrom<&syn::Type> for Type {
    type Error = Error;

    fn try_from(ty: &syn::Type) -> Result<Self, Self::Error> {
        let byte_vec_path: Path = parse_quote!(Vec<u8>);

        let error = || Err(Error::new_spanned(&ty, "unexpected type"));

        match ty {
            syn::Type::Path(p) => {
                if p.path.is_ident("String") {
                    Ok(Type {
                        ty: Types::Str,
                        optional: false,
                    })
                } else if p.path == byte_vec_path {
                    Ok(Type {
                        ty: Types::ByteStr,
                        optional: false,
                    })
                } else if p.path.is_ident("u8") {
                    Ok(Type {
                        ty: Types::Byte,
                        optional: false,
                    })
                } else if p.path.is_ident("char") {
                    Ok(Type {
                        ty: Types::Char,
                        optional: false,
                    })
                } else if p.path.is_ident("i32") {
                    Ok(Type {
                        ty: Types::I32,
                        optional: false,
                    })
                } else if p.path.is_ident("f32") {
                    Ok(Type {
                        ty: Types::F32,
                        optional: false,
                    })
                } else if p.path.is_ident("bool") {
                    Ok(Type {
                        ty: Types::Bool,
                        optional: false,
                    })
                } else {
                    if p.path.segments.len() == 1 {
                        let segment = p.path.segments.first().unwrap();
                        if segment.ident == "Option" {
                            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                                if let Some(GenericArgument::Type(ty)) = args.args.first() {
                                    return Type::try_from(ty).and_then(|ty| {
                                        if ty.optional {
                                            error()
                                        } else {
                                            Ok(Type {
                                                ty: ty.ty,
                                                optional: true,
                                            })
                                        }
                                    });
                                }
                            }
                        }
                    }
                    error()
                }
            }
            syn::Type::Tuple(t) if t.elems.is_empty() => Ok(Type {
                ty: Types::Flag,
                optional: false,
            }),
            _ => error(),
        }
    }
}
