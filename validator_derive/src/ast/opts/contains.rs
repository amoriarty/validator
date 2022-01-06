use darling::FromMeta;

use crate::ast::opts::Opt;

#[derive(Debug, FromMeta)]
pub(crate) struct Contains {
    pub(crate) pattern: String,
    #[darling(default)]
    pub(crate) code: Option<String>,
    #[darling(default)]
    pub(crate) message: Option<String>,
}

impl Opt for Contains {
    fn get_function(&self) -> String {
        "::validator::validation::contains::validate".to_string()
    }

    fn get_arg(&self) -> String {
        format!("({})", self.pattern)
    }

    fn get_code(&self) -> String {
        self.code.clone().unwrap_or_else(|| "contains".to_string())
    }

    fn get_message(&self) -> Option<String> {
        self.message.clone()
    }
}

impl From<String> for Contains {
    fn from(value: String) -> Self {
        Self { pattern: value, code: Option::default(), message: Option::default() }
    }
}
