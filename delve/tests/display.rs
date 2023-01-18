use delve::EnumDisplay;

#[test]
fn test() {
    #[derive(Debug, PartialEq, EnumDisplay)]
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

    assert_eq!("Sunday", format!("{}", Week::Sunday));
    assert_eq!("Wednesday", format!("{}", Week::Wednesday));
}

#[test]
fn test_attrs() {
    fn other(is: &bool) -> String {
        format!("tuesday? {}", is)
    }

    #[derive(Debug, PartialEq, EnumDisplay)]
    #[delve(rename_variants = "uppercase")]
    #[allow(dead_code)]
    enum Week {
        #[delve(display = "sunday")]
        Sunday,
        Monday,
        #[delve(display = other)]
        Tuesday {
            is: bool,
        },
        Wednesday,
        Thursday,
        #[delve(display = || "fri_day")]
        Friday,

        #[delve(display = |b: &bool, i: &usize| format!("saturday? {} {}", b, i))]
        Saturday(bool, usize),
    }

    assert_eq!("MONDAY", format!("{}", Week::Monday));
    assert_eq!("sunday", format!("{}", Week::Sunday));

    assert_eq!("fri_day", format!("{}", Week::Friday));

    assert_eq!("tuesday? true", format!("{}", Week::Tuesday { is: true }));
    assert_eq!(
        "saturday? false 100",
        format!("{}", Week::Saturday(false, 100))
    );
}
