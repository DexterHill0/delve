use delve::ModifyField;
use delve_derive::EnumModify;

#[test]
fn test_plain() {
    #[derive(Debug, PartialEq, EnumModify)]
    #[allow(dead_code)]
    enum Week {
        Sunday,
        Monday,
        Tuesday,
        Wednesday(String),
        Thursday { good: bool, foo: usize },
        Friday,
        Saturday,
    }

    let mut wed = Week::Wednesday("true".into());

    assert_eq!(Some(&"true".into()), wed.get_field(0));

    wed.set_field(0, "false".into());

    assert_eq!(Some(&"false".into()), wed.get_field(0));

    assert_eq!(None, wed.get_field(100));

    /////

    let mut thurs = Week::Thursday {
        good: false,
        foo: 0,
    };

    assert_eq!(Some(&false), thurs.get_field("good"));

    thurs.set_field("good", true);

    assert_eq!(Some(&true), thurs.get_field("good"));

    assert_eq!(
        None,
        <Week as ModifyField<&str, bool>>::get_field(&thurs, "bar")
    );
}

#[test]
fn test_attrs() {
    #[derive(Debug, PartialEq, EnumModify)]
    #[allow(dead_code)]
    #[delve(rename_fields = "uppercase")]
    enum Week<'a, T> {
        Sunday(String, #[delve(skip)] &'a T),
        Monday,
        Tuesday,
        Wednesday(String),
        Thursday {
            #[delve(skip)]
            good: bool,
            #[delve(rename_field = "foo")]
            foo: usize,
            bar: bool,
        },
        Friday,
        Saturday,
    }

    let sun: Week<bool> = Week::Sunday("true".into(), &true);

    assert_eq!(Some(&"true".into()), sun.get_field(0));

    assert_eq!(None, sun.get_field(1));

    /////

    let thurs: Week<bool> = Week::Thursday {
        good: false,
        foo: 0,
        bar: true,
    };

    assert_eq!(Some(&0), thurs.get_field("foo"));

    assert_eq!(Some(&true), thurs.get_field("BAR"));

    assert_eq!(
        None,
        <Week<bool> as ModifyField<&str, bool>>::get_field(&thurs, "bar")
    );
}

#[test]
fn test_resolve() {
    type Foo = u16;
    type Bar = u16;

    #[derive(Debug, PartialEq, EnumModify)]
    #[allow(dead_code)]
    #[delve(rename_fields = "uppercase")]
    #[delve(resolve(Foo, u16), resolve(Bar, u16))]
    enum Week {
        Sunday,
        Monday { a: Foo, b: Bar },
        Tuesday,
        Thursday,
        Friday,
        Saturday,
    }
}
