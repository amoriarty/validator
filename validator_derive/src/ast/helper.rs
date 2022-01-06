use std::fmt::{Display, Formatter};

use darling::FromMeta;
use syn::{Lit, NestedMeta};

#[derive(Debug)]
pub(crate) struct Str<T>(pub(crate) T);

impl<T> FromMeta for Str<T>
where
    T: FromMeta,
    T: From<String>,
{
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        Ok(Self(T::from_list(items)?))
    }

    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(Self(T::from(value.to_string())))
    }
}

#[derive(Debug)]
pub(crate) enum ValueOrPath<T> {
    Value(T),
    Path(String),
}

impl<T> FromMeta for ValueOrPath<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    fn from_value(value: &Lit) -> darling::Result<Self> {
        match value {
            Lit::Int(lit_int) => Ok(Self::Value(lit_int.base10_parse().unwrap())),
            Lit::Str(lit_str) => Self::from_string(&lit_str.value()),
            _ => Err(darling::Error::unexpected_lit_type(value)),
        }
    }

    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(Self::Path(value.to_string()))
    }
}

impl<T> Display for ValueOrPath<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueOrPath::Value(value) => write!(f, "{}", value),
            ValueOrPath::Path(path) => write!(f, "{}", path),
        }
    }
}
