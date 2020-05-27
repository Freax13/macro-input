use syn::{Lit, Meta};

/// a trait for extracting a value from `Option<syn::Meta>`
pub trait FromMeta {
    /// extract the value
    fn from(meta: Option<Meta>) -> Self;
}

impl FromMeta for Option<()> {
    fn from(meta: Option<Meta>) -> Self {
        meta.map(|m| {
            if !matches!(m, Meta::Path(_)) {
                unreachable!()
            }
        })
    }
}

/// a trait for extracting a value from a literal
///
/// [`FromMeta`] is automatically implemented for all implementations
pub trait FromLit {
    /// extract the value
    fn from(lit: Option<Lit>) -> Self;
}

impl<F: FromLit> FromMeta for F {
    fn from(meta: Option<Meta>) -> Self {
        let lit = meta.map(|m| match m {
            Meta::NameValue(mnv) => mnv.lit,
            _ => unreachable!(),
        });
        Self::from(lit)
    }
}

impl FromLit for Option<Lit> {
    fn from(lit: Option<Lit>) -> Self {
        lit
    }
}

impl FromLit for String {
    fn from(lit: Option<Lit>) -> Self {
        if let Some(Lit::Str(v)) = lit {
            v.value()
        } else {
            unreachable!()
        }
    }
}

impl FromLit for Vec<u8> {
    fn from(lit: Option<Lit>) -> Self {
        if let Some(Lit::ByteStr(v)) = lit {
            v.value()
        } else {
            unreachable!()
        }
    }
}

impl FromLit for u8 {
    fn from(lit: Option<Lit>) -> Self {
        if let Some(Lit::Byte(v)) = lit {
            v.value()
        } else {
            unreachable!()
        }
    }
}

impl FromLit for char {
    fn from(lit: Option<Lit>) -> Self {
        if let Some(Lit::Char(v)) = lit {
            v.value()
        } else {
            unreachable!()
        }
    }
}

impl FromLit for i32 {
    fn from(lit: Option<Lit>) -> Self {
        if let Some(Lit::Int(v)) = lit {
            v.base10_parse().unwrap()
        } else {
            unreachable!()
        }
    }
}

impl FromLit for f32 {
    fn from(lit: Option<Lit>) -> Self {
        if let Some(Lit::Float(v)) = lit {
            v.base10_parse().unwrap()
        } else {
            unreachable!()
        }
    }
}

impl FromLit for bool {
    fn from(lit: Option<Lit>) -> Self {
        if let Some(Lit::Bool(v)) = lit {
            v.value
        } else {
            unreachable!()
        }
    }
}

impl<V: FromLit> FromLit for Option<V> {
    fn from(lit: Option<Lit>) -> Self {
        if lit.is_some() {
            Some(V::from(lit))
        } else {
            None
        }
    }
}
