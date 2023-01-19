use delve::TupleCount;
use delve_derive::EnumTuples;

#[derive(Debug, PartialEq, EnumTuples)]
#[allow(dead_code)]
enum Week {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday(bool, bool),
    Friday,
    Saturday,
}

#[test]
fn test() {
    assert_eq!(None, Week::Sunday.tuple_count());
    assert_eq!(Some(2), Week::Thursday(false, false).tuple_count());
}
