use darling::FromMeta;

use crate::ast::opts::Opt;

#[derive(Debug, FromMeta)]
pub(crate) struct Email {
    #[darling(default)]
    code: Option<String>,
    #[darling(default)]
    message: Option<String>,
}

impl Default for Email {
    fn default() -> Self {
        Self { code: Option::default(), message: Option::default() }
    }
}

impl Opt for Email {
    fn get_function(&self) -> String {
        "::validator::validation::email::validate".to_string()
    }

    fn get_arg(&self) -> String {
        "()".to_string()
    }

    fn get_code(&self) -> String {
        self.code.clone().unwrap_or_else(|| "email".to_string())
    }

    fn get_message(&self) -> Option<String> {
        self.message.clone()
    }
}
