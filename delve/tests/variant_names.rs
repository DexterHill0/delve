use delve::{EnumVariantNames, VariantNames};

#[test]
fn test_plain() {
    #[allow(dead_code)]
    #[derive(Debug, EnumVariantNames)]
    enum Week {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
    }

    assert_eq!(
        &[
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ],
        Week::VARIANT_NAMES
    )
}

#[test]
fn test_attrs() {
    #[allow(dead_code)]
    #[derive(Debug, EnumVariantNames)]
    #[delve(rename_variants = "uppercase")] // `rename_variants` takes priority over `rename_all`
    #[delve(rename_all = "lowercase")]
    enum Week {
        #[delve(skip)]
        Sunday,
        Monday,
        Tuesday,
        #[delve(rename_variant = "wed_nes_day")] // `rename_variant` takes priority over both
        Wednesday,
        Thursday,
        Friday,
        #[delve(rename_variant = "SaTuRDay")]
        Saturday,
    }

    assert_eq!(
        &[
            "MONDAY",
            "TUESDAY",
            "wed_nes_day",
            "THURSDAY",
            "FRIDAY",
            "SaTuRDay",
        ],
        Week::VARIANT_NAMES
    )
}
