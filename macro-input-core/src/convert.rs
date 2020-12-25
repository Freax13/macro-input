use syn::{Error, Lit, Meta, Result};

/// a trait for extracting a value from `Option<syn::Meta>`
pub trait FromMeta: Sized {
    /// extract the value
    ///
    /// # Errors
    /// may return an Error if the meta doesn't contain the correct value
    fn from(meta: Option<Meta>) -> Result<Self>;
}

impl FromMeta for Option<()> {
    fn from(meta: Option<Meta>) -> Result<Self> {
        meta.map_or(Ok(None), |m| {
            if matches!(m, Meta::Path(_)) {
                Ok(Some(()))
            } else {
                Err(Error::new_spanned(m, "unexpected value"))
            }
        })
    }
}

/// a trait for extracting a value from a literal
///
/// [`FromMeta`] is automatically implemented for all implementations
pub trait FromLit: Sized {
    /// extract the value
    ///
    /// # Errors
    /// may return an Error if the literal doesn't contain the correct value
    fn from(lit: Option<Lit>) -> Result<Self>;
}

impl<F: FromLit> FromMeta for F {
    fn from(meta: Option<Meta>) -> Result<Self> {
        let lit = meta
            .map(|m| match m {
                Meta::NameValue(mnv) => Ok(mnv.lit),
                _ => Err(Error::new_spanned(m, "expected named value")),
            })
            .transpose()?;
        Self::from(lit)
    }
}

impl FromLit for Option<Lit> {
    fn from(lit: Option<Lit>) -> Result<Self> {
        Ok(lit)
    }
}

impl FromLit for String {
    fn from(lit: Option<Lit>) -> Result<Self> {
        if let Some(Lit::Str(v)) = lit {
            Ok(v.value())
        } else {
            Err(Error::new_spanned(lit, "expected string"))
        }
    }
}

impl FromLit for Vec<u8> {
    fn from(lit: Option<Lit>) -> Result<Self> {
        if let Some(Lit::ByteStr(v)) = lit {
            Ok(v.value())
        } else {
            Err(Error::new_spanned(lit, "expected bytes"))
        }
    }
}

impl FromLit for u8 {
    fn from(lit: Option<Lit>) -> Result<Self> {
        if let Some(Lit::Byte(v)) = lit {
            Ok(v.value())
        } else {
            Err(Error::new_spanned(lit, "expected byte"))
        }
    }
}

impl FromLit for char {
    fn from(lit: Option<Lit>) -> Result<Self> {
        if let Some(Lit::Char(v)) = lit {
            Ok(v.value())
        } else {
            Err(Error::new_spanned(lit, "expected char"))
        }
    }
}

impl FromLit for i32 {
    fn from(lit: Option<Lit>) -> Result<Self> {
        if let Some(Lit::Int(v)) = &lit {
            v.base10_parse()
        } else {
            Err(Error::new_spanned(lit, "expected i32"))
        }
    }
}

impl FromLit for f32 {
    fn from(lit: Option<Lit>) -> Result<Self> {
        if let Some(Lit::Float(v)) = &lit {
            v.base10_parse()
        } else {
            Err(Error::new_spanned(lit, "expected f32"))
        }
    }
}

impl FromLit for bool {
    fn from(lit: Option<Lit>) -> Result<Self> {
        if let Some(Lit::Bool(v)) = lit {
            Ok(v.value)
        } else {
            Err(Error::new_spanned(lit, "expected bool"))
        }
    }
}

impl<V: FromLit> FromLit for Option<V> {
    fn from(lit: Option<Lit>) -> Result<Self> {
        if lit.is_some() {
            Some(V::from(lit)).transpose()
        } else {
            Ok(None)
        }
    }
}
