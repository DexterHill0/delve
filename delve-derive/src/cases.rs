use darling::FromMeta;

use crate::utils::syn_err;

#[derive(Copy, Clone, Debug)]
pub(crate) enum Inflection {
    Lower,
    Upper,
    Camel,
    Snake,
    Pascal,
    ScreamingSnake,
    Kebab,
}

impl FromMeta for Inflection {
    fn from_string(value: &str) -> darling::Result<Self> {
        Inflection::try_from(value.to_string()).map_err(|e| e.into())
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
