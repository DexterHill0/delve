use delve::{EnumFields, FieldNames};

#[test]
fn test_plain() {
    #[derive(Debug, EnumFields)]
    #[allow(dead_code)]
    enum Week {
        Sunday,
        Monday,
        Tuesday,
        Wednesday { foo: bool, bar: usize },
        Thursday,
        Friday,
        Saturday,
    }

    assert_eq!(None, Week::Monday.field_names());
    assert_eq!(
        Some(&["foo", "bar"][..]),
        Week::Wednesday { foo: true, bar: 0 }.field_names()
    );
}

#[test]
fn test_attrs() {
    #[derive(Debug, EnumFields)]
    #[allow(dead_code)]
    #[delve(rename_fields = "uppercase")]
    enum Week {
        Sunday,
        Monday,
        Tuesday,
        Wednesday {
            foo: bool,
            bar: usize,
        },
        Thursday,
        Friday {
            #[delve(skip)]
            a: bool,
            #[delve(rename_field = "bb")]
            b: bool,
        },
        Saturday,
    }

    assert_eq!(None, Week::Monday.field_names());
    assert_eq!(
        Some(&["FOO", "BAR"][..]),
        Week::Wednesday { foo: true, bar: 0 }.field_names()
    );
    assert_eq!(
        Some(&["bb"][..]),
        Week::Friday { a: true, b: false }.field_names()
    );
}

// TODO: check scoping of `display`!
