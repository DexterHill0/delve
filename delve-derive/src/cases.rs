use deluxe::ParseMetaItem;
use syn::LitStr;

use crate::utils::syn_err;

/// The different kinds of cases that am ident can be converted to.
#[derive(Copy, Clone, Debug)]
pub enum Inflection {
    Lower,
    Upper,
    Camel,
    Snake,
    Pascal,
    ScreamingSnake,
    Kebab,
}

impl ParseMetaItem for Inflection {
    fn parse_meta_item(
        input: deluxe::____private::ParseStream,
        _mode: deluxe::ParseMode,
    ) -> deluxe::Result<Self> {
        let string: LitStr = input.parse()?;

        Inflection::try_from(string.value()).map_err(|e| e.into())
    }
}

impl Inflection {
    pub fn apply(self, string: &str) -> String {
        use inflector::Inflector;

        match self {
            Inflection::Lower => string.to_lowercase(),
            Inflection::Upper => string.to_uppercase(),
            Inflection::Camel => string.to_camel_case(),
            Inflection::Snake => string.to_snake_case(),
            Inflection::Pascal => string.to_pascal_case(),
            Inflection::ScreamingSnake => string.to_screaming_snake_case(),
            Inflection::Kebab => string.to_kebab_case(),
        }
    }
}

impl TryFrom<String> for Inflection {
    type Error = syn::Error;

    fn try_from(value: String) -> syn::Result<Self> {
        Ok(
            match &*value.to_lowercase().replace('_', "").replace('-', "") {
                "lowercase" => Self::Lower,
                "uppercase" => Self::Upper,
                "camelcase" => Self::Camel,
                "snakecase" => Self::Snake,
                "pascalcase" => Self::Pascal,
                "screamingsnakecase" => Self::ScreamingSnake,
                "kebabcase" => Self::Kebab,
                _ => syn_err!("invalid inflection: '{}'", value),
            },
        )
    }
}
