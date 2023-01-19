use delve::VariantCount;
use delve_derive::EnumVariantCount;

#[derive(Debug, EnumVariantCount)]
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
    assert_eq!(7, Week::VARIANT_COUNT);
}
