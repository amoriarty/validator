use darling::FromMeta;

use crate::ast::opts::Opt;

#[derive(Debug, FromMeta)]
pub(crate) struct Required {
    #[darling(default)]
    pub(crate) code: Option<String>,
    #[darling(default)]
    pub(crate) message: Option<String>,
}

impl Default for Required {
    fn default() -> Self {
        Self { code: Option::default(), message: Option::default() }
    }
}

impl Opt for Required {
    fn get_function(&self) -> String {
        "::validator::validation::required::validate".to_string()
    }

    fn get_arg(&self) -> String {
        "()".to_string()
    }

    fn get_code(&self) -> String {
        self.code.clone().unwrap_or_else(|| "required".to_string())
    }

    fn get_message(&self) -> Option<String> {
        self.message.clone()
    }
}
