# Delve

Delve provides a number of macros that make working with enums and strings more convenient.

[<img alt="github" src="https://img.shields.io/badge/github-dexterhill0/delve-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/DexterHill0/delve) [<img alt="crates.io" src="https://img.shields.io/crates/v/delve.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/delve) [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-delve-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/delve)

# Including Delve

`delve` can be included using:

```toml
[dependencies]
delve = { version = "0.2.0", features = ["derive"] }
```

# Derive Macros

| Macro              | Description                                                                          |
| ------------------ | ------------------------------------------------------------------------------------ |
| [EnumVariantCount] | Adds an associated `VARIANT_COUNT` for getting the number of variants in an enum.    |
| [EnumVariantNames] | Adds an associated `VARIANT_NAMES` for getting the names of the variants in an enum. |
| [EnumHasVariant]   | Returns whether a given variant name exists in the enum.                             |
| [EnumDisplay]      | Converts an enum variant to a string.                                                |
| [EnumFromStr]      | Converts a string to an enum variant                                                 |
| [EnumFields]       | Returns the field names from within a struct variant.                                |
| [EnumTuples]       | Returns the number of types within a tuple variant.                                  |
| [EnumModify]       | Allows the modification of arguments within a tuple or struct variant.               |
| [EnumToStr]        | Converts an enum variant to a string.                                                |

[enumvariantcount]: https://docs.rs/delve/0.1.0/delve/derive.EnumVariantCount.html
[enumvariantnames]: https://docs.rs/delve/0.1.0/delve/derive.EnumVariantNames.html
[enumhasvariant]: https://docs.rs/delve/0.1.0/delve/derive.EnumHasVariant.html
[enumdisplay]: https://docs.rs/delve/0.1.0/delve/derive.EnumDisplay.html
[enumfromstr]: https://docs.rs/delve/0.1.0/delve/derive.EnumFromStr.html
[enumfields]: https://docs.rs/delve/0.1.0/delve/derive.EnumFields.html
[enumtuples]: https://docs.rs/delve/0.1.0/delve/derive.EnumTuples.html
[enummodify]: https://docs.rs/delve/0.1.0/delve/derive.EnumModify.html
[enumtostr]: https://docs.rs/delve/0.1.0/delve/derive.EnumToStr.html
