use darling::FromMeta;

use crate::ast::opts::Opt;

#[derive(Debug, FromMeta)]
pub(crate) struct Schema {
    pub(crate) function: String,
    #[darling(default)]
    pub(crate) arg: Option<String>,
    #[darling(default)]
    pub(crate) skip_on_field_errors: Option<bool>,
    #[darling(default)]
    pub(crate) code: Option<String>,
    #[darling(default)]
    pub(crate) message: Option<String>,
}

impl Opt for Schema {
    fn get_function(&self) -> String {
        self.function.clone()
    }

    fn get_arg(&self) -> String {
        self.arg.clone().unwrap_or_else(|| "()".to_string())
    }

    fn get_code(&self) -> String {
        self.code.clone().unwrap_or_else(|| "non_control_character".to_string())
    }

    fn get_message(&self) -> Option<String> {
        self.message.clone()
    }
}
