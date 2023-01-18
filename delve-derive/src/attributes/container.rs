use darling::FromDeriveInput;

use crate::cases::Inflection;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(delve))]
pub(crate) struct EnumAttribute {
    pub rename_all: Option<Inflection>,
    pub rename_fields: Option<Inflection>,
    pub rename_variants: Option<Inflection>,
}
