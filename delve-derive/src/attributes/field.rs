use deluxe::ParseAttributes;

#[derive(Debug, ParseAttributes)]
#[deluxe(attributes(delve))]
pub(crate) struct FieldAttribute {
    #[deluxe(default)]
    pub rename_field: Option<String>,

    #[deluxe(default)]
    pub skip: bool,
}
