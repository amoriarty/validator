use darling::FromMeta;

use crate::ast::opts::Opt;

#[derive(Debug, FromMeta)]
pub(crate) struct Regex {
    pub(crate) path: String,
    #[darling(default)]
    pub(crate) code: Option<String>,
    #[darling(default)]
    pub(crate) message: Option<String>,
}

impl Opt for Regex {
    fn get_function(&self) -> String {
        "::validator::validation::regex::validate".to_string()
    }

    fn get_arg(&self) -> String {
        format!("({})", self.path)
    }

    fn get_code(&self) -> String {
        self.code.clone().unwrap_or_else(|| "regex".to_string())
    }

    fn get_message(&self) -> Option<String> {
        self.message.clone()
    }
}

impl From<String> for Regex {
    fn from(value: String) -> Self {
        Self { path: value.to_string(), code: Option::default(), message: Option::default() }
    }
}
