use darling::FromMeta;

use crate::ast::opts::Opt;

#[derive(Debug, FromMeta)]
pub(crate) struct Custom {
    pub(crate) function: String,
    #[darling(default)]
    pub(crate) arg: Option<String>,
    #[darling(default)]
    pub(crate) code: Option<String>,
    #[darling(default)]
    pub(crate) message: Option<String>,
}

impl Opt for Custom {
    fn get_function(&self) -> String {
        self.function.clone()
    }

    fn get_arg(&self) -> String {
        self.arg.clone().unwrap_or_else(|| "()".to_string())
    }

    fn get_code(&self) -> String {
        self.code.clone().unwrap_or_else(|| "custom".to_string())
    }

    fn get_message(&self) -> Option<String> {
        self.message.clone()
    }
}

impl From<String> for Custom {
    fn from(value: String) -> Self {
        Self {
            function: value,
            arg: Option::default(),
            code: Option::default(),
            message: Option::default(),
        }
    }
}
