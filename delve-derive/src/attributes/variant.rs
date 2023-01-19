use deluxe::{ParseAttributes, ParseMetaItem};
use syn::{ExprClosure, Ident, LitStr, Path, Token};

#[derive(Debug, ParseAttributes)]
#[deluxe(attributes(delve))]
pub(crate) struct VariantAttribute {
    #[deluxe(default)]
    pub skip: bool,

    #[deluxe(default)]
    pub ascii_case_insensitive: bool,

    #[deluxe(default)]
    pub rename_variant: Option<String>,

    #[deluxe(default, append)]
    pub from: Vec<String>,

    #[deluxe(default)]
    pub to: Option<String>,

    #[deluxe(default)]
    pub display: Option<DisplayValue>,
}

#[derive(Debug)]
pub(crate) enum DisplayValue {
    ExternFn(Path),
    String(String),

    Closure(ExprClosure),
}

impl ParseMetaItem for DisplayValue {
    fn parse_meta_item(
        input: deluxe::____private::ParseStream,
        _mode: deluxe::ParseMode,
    ) -> deluxe::Result<Self> {
        if input.peek(Token![|]) {
            Ok(DisplayValue::Closure(input.parse::<ExprClosure>()?))
        } else if input.peek(Ident) || input.peek(Token![::]) {
            Ok(DisplayValue::ExternFn(input.parse::<Path>()?))
        } else {
            Ok(DisplayValue::String(input.parse::<LitStr>()?.value()))
        }
    }
}
