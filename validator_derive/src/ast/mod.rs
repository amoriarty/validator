use cfg_if::cfg_if;
use darling::ast::Data;
use darling::util::Override::{Explicit, Inherit};
use darling::util::{Ignored, Override};
use darling::{FromDeriveInput, FromField};

use opts::contains::Contains;
#[cfg(feature = "card")]
use opts::credit_card::CreditCard;
use opts::custom::Custom;
use opts::email::Email;
use opts::length::Length;
use opts::must_match::MustMatch;
#[cfg(feature = "unic")]
use opts::non_control_character::NonControlCharacter;
#[cfg(feature = "phone")]
use opts::phone::Phone;
use opts::range::Range;
use opts::regex::Regex;
use opts::required::Required;
use opts::required_nested::RequiredNested;
use opts::schema::Schema;
use opts::url::Url;

use crate::ast::helper::Str;
use crate::ast::opts::Opt;

pub(crate) mod helper;
pub(crate) mod opts;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(validate), supports(struct_named))]
pub(crate) struct ValidateInput {
    pub(crate) ident: syn::Ident,
    pub(crate) generics: syn::Generics,
    pub(crate) data: Data<Ignored, ValidateField>,
    #[darling(multiple, rename = "schema")]
    pub(crate) schemas: Vec<Schema>,
}

#[derive(Debug, FromField)]
#[darling(attributes(validate), and_then = "ValidateField::add_validations")]
pub(crate) struct ValidateField {
    pub(crate) ident: Option<syn::Ident>,
    pub(crate) ty: syn::Type,
    #[darling(skip)]
    pub(crate) validations: Vec<Validations>,
    #[darling(multiple)]
    pub(crate) contains: Vec<Str<Contains>>,
    #[cfg(feature = "card")]
    #[darling(multiple)]
    pub(crate) credit_card: Vec<Override<CreditCard>>,
    #[darling(multiple)]
    pub(crate) custom: Vec<Str<Custom>>,
    #[darling(multiple)]
    pub(crate) email: Vec<Override<Email>>,
    // todo ip
    #[darling(multiple)]
    pub(crate) length: Vec<Length>,
    #[darling(multiple)]
    pub(crate) must_match: Vec<Str<MustMatch>>,
    #[cfg(feature = "unic")]
    #[darling(multiple)]
    pub(crate) non_control_character: Vec<Override<NonControlCharacter>>,
    #[cfg(feature = "phone")]
    #[darling(multiple)]
    pub(crate) phone: Vec<Override<Phone>>,
    #[darling(multiple)]
    pub(crate) range: Vec<Range>,
    #[darling(multiple)]
    pub(crate) regex: Vec<Str<Regex>>,
    #[darling(multiple)]
    pub(crate) required: Vec<Override<Required>>,
    #[darling(multiple)]
    pub(crate) required_nested: Vec<Override<RequiredNested>>,
    #[darling(multiple)]
    pub(crate) url: Vec<Override<Url>>,
}

#[derive(Debug)]
pub(crate) struct Validations {
    pub(crate) function: String,
    pub(crate) arg: String,
    pub(crate) code: String,
    pub(crate) message: Option<String>,
}

impl ValidateField {
    fn add_validations(self) -> darling::Result<Self> {
        let mut validations: Vec<Validations> = vec![
            self.contains.iter().map(|v| v.into()).collect::<Vec<Validations>>(),
            self.custom.iter().map(|v| v.into()).collect::<Vec<Validations>>(),
            self.email.iter().map(|v| v.into()).collect::<Vec<Validations>>(),
            self.length.iter().map(|v| v.into()).collect::<Vec<Validations>>(),
            self.must_match.iter().map(|v| v.into()).collect::<Vec<Validations>>(),
            self.range.iter().map(|v| v.into()).collect::<Vec<Validations>>(),
            self.regex.iter().map(|v| v.into()).collect::<Vec<Validations>>(),
            self.required.iter().map(|v| v.into()).collect::<Vec<Validations>>(),
            self.url.iter().map(|v| v.into()).collect::<Vec<Validations>>(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<Validations>>();

        cfg_if! {
            if #[cfg(feature = "card")] {
                validations.append(
                    self.credit_card.iter().map(|v| v.into()).collect::<Vec<Validations>>().as_mut(),
                )
            }
        }

        cfg_if! {
            if #[cfg(feature = "unic")] {
                validations.append(
                    self.non_control_character.iter().map(|v| v.into()).collect::<Vec<Validations>>().as_mut(),
                )
            }
        }

        cfg_if! {
            if #[cfg(feature = "phone")] {
                validations.append(
                    self.phone.iter().map(|v| v.into()).collect::<Vec<Validations>>().as_mut(),
                )
            }
        }

        Ok(Self { validations, ..self })
    }
}

impl<T> From<&T> for Validations
where
    T: opts::Opt,
{
    fn from(value: &T) -> Self {
        Self {
            function: value.get_function(),
            arg: value.get_arg(),
            code: value.get_code(),
            message: value.get_message(),
        }
    }
}

impl<T> From<&Override<T>> for Validations
where
    T: Opt,
    T: Default,
{
    fn from(value: &Override<T>) -> Self {
        match value {
            Inherit => (&T::default()).into(),
            Explicit(value) => value.into(),
        }
    }
}

impl<T> From<&Str<T>> for Validations
where
    T: Opt,
{
    fn from(str: &Str<T>) -> Self {
        (&str.0).into()
    }
}
