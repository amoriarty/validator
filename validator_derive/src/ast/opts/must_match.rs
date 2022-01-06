use darling::FromMeta;

use crate::ast::opts::Opt;

#[derive(Debug, FromMeta)]
pub(crate) struct MustMatch {
    pub(crate) other: String,
    #[darling(default)]
    pub(crate) code: Option<String>,
    #[darling(default)]
    pub(crate) message: Option<String>,
}

impl Opt for MustMatch {
    fn get_function(&self) -> String {
        "::validator::validation::must_match::validation".to_string()
    }

    fn get_arg(&self) -> String {
        format!("({})", self.other)
    }

    fn get_code(&self) -> String {
        self.code.clone().clone().unwrap_or_else(|| "must_match".to_string())
    }

    fn get_message(&self) -> Option<String> {
        self.message.clone()
    }
}

impl From<String> for MustMatch {
    fn from(value: String) -> Self {
        Self { other: value, code: Option::default(), message: Option::default() }
    }
}
