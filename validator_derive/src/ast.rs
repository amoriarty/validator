use darling::ast::Data;
use darling::util::{Ignored, Override};
use darling::{FromDeriveInput, FromField, FromMeta};
use syn::{Lit, NestedMeta};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(validate), supports(struct_named))]
pub(crate) struct ValidateInput {
    pub(crate) ident: syn::Ident,
    pub(crate) generics: syn::Generics,
    pub(crate) data: Data<Ignored, ValidateField>,
    #[darling(multiple, rename = "schema")]
    pub(crate) schemas: Vec<opts::Schema>,
}

#[derive(Debug, FromField)]
#[darling(attributes(validate))]
pub(crate) struct ValidateField {
    pub(crate) ident: Option<syn::Ident>,
    pub(crate) ty: syn::Type,
    #[darling(default)]
    pub(crate) contains: Option<Str<opts::Contains>>,
    #[darling(default)]
    pub(crate) custom: Option<Str<opts::Custom>>,
    #[cfg(feature = "card")]
    #[darling(default)]
    pub(crate) credit_card: Option<Override<Opts>>,
    #[darling(default)]
    pub(crate) email: Option<Override<Opts>>,
    #[darling(default)]
    pub(crate) length: Option<opts::Length>,
    #[darling(default)]
    pub(crate) must_match: Option<Str<opts::MustMatch>>,
    #[cfg(feature = "unic")]
    #[darling(default)]
    pub(crate) non_control_character: Option<Override<Opts>>,
    #[cfg(feature = "phone")]
    #[darling(default)]
    pub(crate) phone: Option<Override<Opts>>,
    #[darling(default)]
    pub(crate) url: Option<Override<Opts>>,
    #[darling(default)]
    pub(crate) range: Option<opts::Range>,
    #[darling(default)]
    pub(crate) required: Option<Override<Opts>>,
    #[darling(default)]
    pub(crate) required_nested: Option<Override<Opts>>,
    #[darling(default)]
    pub(crate) regex: Option<Str<opts::Regex>>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct Opts {
    #[darling(default)]
    code: Option<String>,
    #[darling(default)]
    message: Option<String>,
}

#[derive(Debug)]
pub(crate) struct Str<T>(T);

pub(crate) trait FromStr {
    fn from_str(value: &str) -> Result<Self, darling::Error>
    where
        Self: Sized;
}

impl<T> FromMeta for Str<T>
where
    T: FromMeta,
    T: FromStr,
{
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        Ok(Self(T::from_list(items)?))
    }

    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(Self(T::from_str(value)?))
    }
}

#[derive(Debug)]
pub(crate) enum ValueOrPath {
    Value(u64),
    Path(String),
}

impl FromMeta for ValueOrPath {
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

pub(crate) mod opts {
    use darling::{Error, FromMeta};

    use crate::ast::ValueOrPath;

    use super::FromStr;

    #[derive(Debug, FromMeta)]
    pub(crate) struct Contains {
        pattern: String,
        #[darling(default)]
        code: Option<String>,
        #[darling(default)]
        message: Option<String>,
    }

    impl FromStr for Contains {
        fn from_str(value: &str) -> Result<Self, Error>
        where
            Self: Sized,
        {
            Ok(Self {
                pattern: value.to_string(),
                code: Option::default(),
                message: Option::default(),
            })
        }
    }

    #[derive(Debug, FromMeta)]
    pub(crate) struct Custom {
        function: String,
        #[darling(default)]
        arg: Option<String>,
        #[darling(default)]
        code: Option<String>,
        #[darling(default)]
        message: Option<String>,
    }

    impl FromStr for Custom {
        fn from_str(value: &str) -> Result<Self, darling::Error> {
            Ok(Self {
                function: value.to_string(),
                arg: Option::default(),
                code: Option::default(),
                message: Option::default(),
            })
        }
    }

    #[derive(Debug, FromMeta)]
    pub(crate) struct Length {
        #[darling(default)]
        min: Option<ValueOrPath>,
        #[darling(default)]
        max: Option<ValueOrPath>,
        #[darling(default)]
        equal: Option<ValueOrPath>,
        #[darling(default)]
        code: Option<String>,
        #[darling(default)]
        message: Option<String>,
    }

    #[derive(Debug, FromMeta)]
    pub(crate) struct Range {
        #[darling(default)]
        min: Option<ValueOrPath>,
        #[darling(default)]
        max: Option<ValueOrPath>,
        #[darling(default)]
        code: Option<String>,
        #[darling(default)]
        message: Option<String>,
    }

    #[derive(Debug, FromMeta)]
    pub(crate) struct MustMatch {
        other: String,
        #[darling(default)]
        code: Option<String>,
        #[darling(default)]
        message: Option<String>,
    }

    impl FromStr for MustMatch {
        fn from_str(value: &str) -> Result<Self, Error>
        where
            Self: Sized,
        {
            Ok(Self {
                other: value.to_string(),
                code: Option::default(),
                message: Option::default(),
            })
        }
    }

    #[derive(Debug, FromMeta)]
    pub(crate) struct Regex {
        path: String,
        #[darling(default)]
        code: Option<String>,
        #[darling(default)]
        message: Option<String>,
    }

    impl FromStr for Regex {
        fn from_str(value: &str) -> Result<Self, Error>
        where
            Self: Sized,
        {
            Ok(Self {
                path: value.to_string(),
                code: Option::default(),
                message: Option::default(),
            })
        }
    }

    #[derive(Debug, FromMeta)]
    pub(crate) struct Schema {
        function: String,
        #[darling(default)]
        arg: Option<String>,
        #[darling(default)]
        skip_on_field_errors: Option<bool>,
        #[darling(default)]
        code: Option<String>,
        #[darling(default)]
        message: Option<String>,
    }
}
