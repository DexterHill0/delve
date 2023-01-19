use delve::HasVariant;
use delve_derive::EnumHasVariant;

#[derive(Debug, EnumHasVariant)]
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

#[test]
fn test() {
    assert!(Week::has_variant("Sunday"));
    assert!(Week::has_variant("Tuesday"));

    assert!(!Week::has_variant("friday"));
}
