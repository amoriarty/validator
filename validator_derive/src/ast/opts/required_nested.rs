use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub(crate) struct RequiredNested {
    #[darling(default)]
    pub(crate) code: Option<String>,
    #[darling(default)]
    pub(crate) message: Option<String>,
}
