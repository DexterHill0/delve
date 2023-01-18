use darling::FromAttributes;

#[derive(Debug, FromAttributes)]
#[darling(attributes(delve))]
pub(crate) struct VariantAttribute {
    pub skip: Option<bool>,
    pub default: Option<bool>,
    pub ascii_case_insensitive: Option<bool>,

    pub rename_variant: Option<String>,

    #[darling(multiple)]
    pub from: Vec<String>,
    pub to: Option<String>,
}
