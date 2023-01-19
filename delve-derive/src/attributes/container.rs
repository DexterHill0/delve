use deluxe::ParseAttributes;

use crate::cases::Inflection;

#[derive(Debug, ParseAttributes)]
#[deluxe(attributes(delve))]
pub(crate) struct EnumAttribute {
    #[deluxe(default)]
    pub rename_all: Option<Inflection>,

    #[deluxe(default)]
    pub rename_fields: Option<Inflection>,

    #[deluxe(default)]
    pub rename_variants: Option<Inflection>,

    #[deluxe(default, append)]
    pub resolve: Vec<(syn::Type, syn::Type)>,
}
