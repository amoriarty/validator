use darling::FromMeta;

use crate::ast::opts::Opt;

#[derive(Debug, FromMeta)]
pub(crate) struct Phone {
    #[darling(default)]
    pub(crate) code: Option<String>,
    #[darling(default)]
    pub(crate) message: Option<String>,
}

impl Default for Phone {
    fn default() -> Self {
        Self { code: Option::None, message: Option::None }
    }
}

impl Opt for Phone {
    fn get_function(&self) -> String {
        "::validator::validation::phone::validate".to_string()
    }

    fn get_arg(&self) -> String {
        "()".to_string()
    }

    fn get_code(&self) -> String {
        self.code.clone().unwrap_or_else(|| "phone".to_string())
    }

    fn get_message(&self) -> Option<String> {
        self.message.clone()
    }
}
