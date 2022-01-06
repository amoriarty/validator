use darling::FromMeta;

use crate::ast::opts::Opt;

#[derive(Debug, FromMeta)]
pub(crate) struct CreditCard {
    #[darling(default)]
    code: Option<String>,
    #[darling(default)]
    message: Option<String>,
}

impl Default for CreditCard {
    fn default() -> Self {
        Self { code: Option::default(), message: Option::default() }
    }
}

impl Opt for CreditCard {
    fn get_function(&self) -> String {
        "::validator::validation::cards::validate".to_string()
    }

    fn get_arg(&self) -> String {
        "()".to_string()
    }

    fn get_code(&self) -> String {
        self.code.clone().unwrap_or_else(|| "credit_card".to_string())
    }

    fn get_message(&self) -> Option<String> {
        self.message.clone()
    }
}
