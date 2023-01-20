use delve_derive::EnumToStr;

#[test]
fn test_plain() {
    #[derive(Debug, PartialEq, EnumToStr)]
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

    assert_eq!("Sunday", <Week as Into<&'static str>>::into(Week::Sunday));
    assert_eq!("Friday", <Week as Into<&'static str>>::into(Week::Friday));
}

#[test]
fn test_attrs() {
    #[derive(Debug, PartialEq, EnumToStr)]
    #[allow(dead_code)]
    #[delve(rename_variants = "uppercase")]
    enum Week<'a> {
        Sunday,
        Monday,
        Tuesday,
        #[delve(skip)]
        Wednesday,
        Thursday(&'a String),
        #[delve(to = "fri")]
        Friday,
        Saturday,
    }

    assert_eq!("MONDAY", <Week as Into<&'static str>>::into(Week::Monday));
    assert_eq!("fri", <Week as Into<&'static str>>::into(Week::Friday));

    assert_eq!("fri", <&Week as Into<&'static str>>::into(&Week::Friday));
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn test_skipped() {
    #[derive(Debug, PartialEq, EnumToStr)]
    #[allow(dead_code)]
    #[delve(rename_variants = "uppercase")]
    enum Week {
        Sunday,
        Monday,
        Tuesday,
        #[delve(skip)]
        Wednesday,
        Thursday,
        #[delve(to = "fri")]
        Friday,
        Saturday,
    }

    <Week as Into<&'static str>>::into(Week::Wednesday);
}
