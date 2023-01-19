use deluxe::ParseAttributes;

#[derive(Debug, ParseAttributes)]
#[deluxe(attributes(delve))]
pub(crate) struct TupleAttribute {
    #[deluxe(default)]
    pub skip: bool,
}
