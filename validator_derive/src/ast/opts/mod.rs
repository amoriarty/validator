pub(crate) mod contains;
#[cfg(feature = "card")]
pub(crate) mod credit_card;
pub(crate) mod custom;
pub(crate) mod email;
pub(crate) mod length;
pub(crate) mod must_match;
#[cfg(feature = "unic")]
pub(crate) mod non_control_character;
#[cfg(feature = "phone")]
pub(crate) mod phone;
pub(crate) mod range;
pub(crate) mod regex;
pub(crate) mod required;
pub(crate) mod required_nested;
pub(crate) mod schema;
pub(crate) mod url;

pub(crate) trait Opt {
    fn get_function(&self) -> String;
    fn get_arg(&self) -> String;
    fn get_code(&self) -> String;
    fn get_message(&self) -> Option<String>;
}
