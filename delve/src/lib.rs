pub use delve_derive::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ParseError {
    VariantNotFound,
}

// core::str::FromStr
// core::convert::From<enum> for &'static str

pub trait VariantCount {
    const VARIANT_COUNT: usize;
}

pub trait VariantNames {
    const VARIANT_NAMES: &'static [&'static str];
}

pub trait HasVariant {
    fn has_variant(variant: &str) -> bool;
}

pub trait FieldNames {
    fn field_names(&self) -> Option<&'static [&'static str]>;
}

pub trait TupleCount {
    fn tuple_count(&self) -> Option<usize>;
}

pub trait ModifyField<F, V> {
    fn get_field(&self, field: F) -> Option<&V>;
    fn set_field(&mut self, field: F, value: V) -> Option<()>;
}

// EnumVariantCount, EnumVariantNames, EnumFromString, EnumFields, EnumTuples, EnumToStr, EnumHasVariant, EnumDiscriminant
// Enum

// #[derive()]
// #[blah(rename_all = "snake_case")]
// #[blah(rename_variants = "snake_case")]
// #[blah(rename_fields = "snake_case")]
// enum Foo {
//     #[blah(rename_variant = "bar")]
//     #[blah(from = "blue", from = "b")]
//     #[blah(to = "__foo")]
//     Foo,

//     #[blah(skip)]
//     #[blah(ascii_case_insensitive)]
//     Bar(String, String),

//     Ham {
//         #[blah(rename_field = "foo")]
//         a: bool,

//         #[blah(skip)]
//         b: usize,
//     },
// }

// fn test() {
//     let f = Foo::Ham { a: false, b: 0 };
//     f.field_names();

//     f.get_field("a");
//     f.set_field("a", value);

//     let f = Foo::Bar("".into(), "".into());

//     f.tuple_count(); //2

//     f.get_field(0);
//     f.set_field(1, value);
// }

// enum Test {
//     A,
//     B(usize, String),
//     C { d: bool },
// }

// impl ModifyField<usize, usize> for Test {
//     fn get_field(&self, field: usize) -> Option<&usize> {
//         match self {
//             Self::B(ref a, ..) => match field {
//                 0 => Some(a),
//                 _ => None,
//             },
//             _ => None,
//         }
//     }
//     fn set_field(&mut self, field: usize, value: usize) -> Option<()> {
//         match self {
//             Self::B(ref mut a, ..) => match field {
//                 0 => {
//                     *a = value;
//                     Some(())
//                 }
//                 _ => None,
//             },
//             _ => None,
//         }
//     }
// }

// impl ModifyField<usize, String> for Test {
//     fn get_field(&self, field: usize) -> Option<&String> {
//         match self {
//             Self::B(.., ref a) => match field {
//                 0 => Some(a),
//                 _ => None,
//             },
//             _ => None,
//         }
//     }
//     fn set_field(&mut self, field: usize, value: String) -> Option<()> {
//         match self {
//             Self::B(.., ref mut a) => match field {
//                 0 => {
//                     *a = value;
//                     Some(())
//                 }
//                 _ => None,
//             },
//             _ => None,
//         }
//     }
// }

// impl ModifyField<String, bool> for Test {
//     fn get_field(&self, field: String) -> Option<&bool> {
//         match self {
//             Self::C { ref d, .. } => match &field[..] {
//                 "d" => Some(&d),
//                 _ => None,
//             },
//             _ => None,
//         }
//     }
//     fn set_field(&mut self, field: String, value: bool) -> Option<()> {
//         match self {
//             Self::C { ref mut d, .. } => match &field[..] {
//                 "d" => {
//                     *d = value;
//                     Some(())
//                 }
//                 _ => None,
//             },
//             _ => None,
//         }
//     }
// }
