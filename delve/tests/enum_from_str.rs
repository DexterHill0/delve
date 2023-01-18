use std::str::FromStr;

use delve::EnumFromString;

#[test]
fn test_plain() {
    #[derive(Debug, PartialEq, EnumFromString)]
    #[allow(dead_code)]
    enum Week {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
    }

    assert_eq!(Ok(Week::Sunday), Week::from_str("Sunday"));
    assert_eq!(Ok(Week::Tuesday), Week::from_str("Tuesday"));
}

#[test]
fn test_attrs() {
    #[derive(Debug, PartialEq, EnumFromString)]
    #[allow(dead_code)]
    #[delve(rename_variants = "uppercase")]
    enum Week {
        #[delve(ascii_case_insensitive)]
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        #[delve(from = "friday", from = "fri")]
        Friday,
        Saturday,
    }

    assert_eq!(Ok(Week::Sunday), Week::from_str("sunday"));
    assert_eq!(Ok(Week::Sunday), Week::from_str("SuNdAy"));

    assert_eq!(Ok(Week::Tuesday), Week::from_str("TUESDAY"));

    assert_eq!(Ok(Week::Friday), Week::from_str("friday"));
    assert_eq!(Ok(Week::Friday), Week::from_str("fri"));
}
