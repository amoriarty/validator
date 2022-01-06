use darling::FromMeta;

use crate::ast::helper::ValueOrPath;
use crate::ast::opts::Opt;

#[derive(Debug, FromMeta)]
pub(crate) struct Range {
    #[darling(default)]
    pub(crate) min: Option<ValueOrPath<u64>>,
    #[darling(default)]
    pub(crate) max: Option<ValueOrPath<u64>>,
    #[darling(default)]
    pub(crate) code: Option<String>,
    #[darling(default)]
    pub(crate) message: Option<String>,
}

impl Opt for Range {
    fn get_function(&self) -> String {
        "::validator::validation::range::validate".to_string()
    }

    fn get_arg(&self) -> String {
        format!("({:?}, {:?})", self.min, self.max)
    }

    fn get_code(&self) -> String {
        self.code.clone().unwrap_or_else(|| "range".to_string())
    }

    fn get_message(&self) -> Option<String> {
        self.message.clone()
    }
}
