<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/serde_yml/images/logos/serde_yml.webp"
alt="Serde YML logo" width="261" align="right" />

<!-- markdownlint-enable MD033 MD041 -->
# Serde YML: Seamless YAML Serialization for Rust

[`Serde YML`][00] is a Rust library that simplifies YAML serialization and deserialization using the popular [Serde][01] framework. It provides a convenient and efficient way to convert Rust data structures to YAML format and vice versa.

## Credits and Acknowledgements

This new library draws inspiration from the excellent work done by [David Tolnay][16] and the maintainers of the [serde-yaml][17] library. While Serde YML started as a fork of serde-yaml, it has now evolved into a separate library with its own goals and direction in mind.

If you are currently using serde-yaml in your projects, we recommend carefully evaluating your requirements and considering the stability and maturity of the original library before migrating to Serde YML.

Finally, I would like to express my sincere gratitude to [David Tolnay][16] and the [serde-yaml][17] team for their valuable contributions to the Rust community and for inspiring this project.

![divider][divider]

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

![Banner of Serde YML][banner]

[![Made With Rust][made-with-rust-badge]][13] [![Crates.io][crates-badge]][08] [![Lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][09] [![License][license-badge]][03] [![Codecov][codecov-badge]][14]

• [Website][00] • [Documentation][09] • [Report Bug][04] • [Request Feature][04] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

<!-- markdownlint-enable MD033 -->

## Features

- Serialization and deserialization of Rust data structures to/from YAML format
- Support for custom structs and enums using Serde's derive macros
- Handling of YAML's `!tag` syntax for representing enum variants
- Direct access to YAML values through the `Value` type and related types like `Mapping` and `Sequence`
- Comprehensive error handling with `Error`, `Location`, and `Result` types
- Serialization to YAML using `to_string` and `to_writer` functions
- Deserialization from YAML using `from_str`, `from_slice`, and `from_reader` functions
- Customizable serialization and deserialization behavior using Serde's `#[serde(with = ...)]` attribute
- Support for serializing/deserializing enums using a YAML map with a single key-value pair through the `singleton_map` module
- Recursive application of `singleton_map` serialization/deserialization to all enums within a data structure using the `singleton_map_recursive` module
- Serialization and deserialization of optional enum fields using the `singleton_map_optional` module
- Handling of nested enum structures with optional inner enums using the `singleton_map_recursive` module
- Customization of serialization and deserialization logic for enums using the `singleton_map_with` module and custom helper functions

### Rust Version Compatibility

This library is compatible with Rust 1.60 and above.

## Installation

Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
serde_yml = "0.0.5"
```

## Usage

Serde YML offers a straightforward and intuitive API for working with YAML data in Rust. Here's a quick example of how to serialize and deserialize a Rust type:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

fn main() -> Result<(), serde_yml::Error> {
    let point = Point { x: 1.0, y: 2.0 };

    // Serialize to YAML
    let yaml = serde_yml::to_string(&point)?;
    assert_eq!(yaml, "x: 1.0\ny: 2.0\n");

    // Deserialize from YAML
    let deserialized_point: Point = serde_yml::from_str(&yaml)?;
    assert_eq!(point, deserialized_point);

    Ok(())
}
```

## Examples

Serde YML provides a set of comprehensive examples to demonstrate its usage and capabilities. You can find them in the `examples` directory of the project.

To run the examples, clone the repository and execute the following command in your terminal from the project root directory:

```shell
cargo run --example example
```

The examples cover various scenarios, including serializing and deserializing structs, enums, optional fields, custom structs, and more.


Here are a few notable examples:

### Serializing and Deserializing Structs

```rust
use serde::{Serialize, Deserialize};
use serde_yml;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() -> Result<(), serde_yml::Error> {
    let point = Point { x: 1.0, y: 2.0 };

    // Serialize to YAML
    let yaml = serde_yml::to_string(&point)?;
    assert_eq!(yaml, "x: 1.0\ny: 2.0\n");

    // Deserialize from YAML
    let deserialized_point: Point = serde_yml::from_str(&yaml)?;
    assert_eq!(point, deserialized_point);

    Ok(())
}
```
This example demonstrates how to serialize and deserialize a simple struct `Point` to and from YAML using the `serde_yml` crate.

![divider][divider]

### Serializing and Deserializing Enums

```rust
use serde::{Serialize, Deserialize};
use serde_yml;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Shape {
    Rectangle { width: u32, height: u32 },
    Circle { radius: f64 },
    Triangle { base: u32, height: u32 },
}

fn main() -> Result<(), serde_yml::Error> {
    let shapes = vec![
        Shape::Rectangle { width: 10, height: 20 },
        Shape::Circle { radius: 5.0 },
        Shape::Triangle { base: 8, height: 12 },
    ];

    // Serialize to YAML
    let yaml = serde_yml::to_string(&shapes)?;
    println!("Serialized YAML:\n{}", yaml);

    // Deserialize from YAML
    let deserialized_shapes: Vec<Shape> = serde_yml::from_str(&yaml)?;
    assert_eq!(shapes, deserialized_shapes);

    Ok(())
}
```

This example demonstrates how to serialize and deserialize an enum `Shape` (with struct variants) to and from YAML using the `serde_yml` crate.

![divider][divider]

### Serializing and Deserializing Optional Fields

```rust
use serde::{Serialize, Deserialize};
use serde_yml;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct User {
    name: String,
    age: Option<u32>,
    #[serde(default)]
    is_active: bool,
}

fn main() -> Result<(), serde_yml::Error> {
    let user = User {
        name: "John".to_string(),
        age: Some(30),
        is_active: true,
    };

    // Serialize to YAML
    let yaml = serde_yml::to_string(&user)?;
    println!("Serialized YAML:\n{}", yaml);

    // Deserialize from YAML
    let deserialized_user: User = serde_yml::from_str(&yaml)?;
    assert_eq!(user, deserialized_user);

    Ok(())
}
```

This example demonstrates how to serialize and deserialize a struct `User` with an optional field `age` to and from YAML using the `serde_yml` crate.

![divider][divider]

### Serializing and Deserializing a HashMap

```rust
use std::collections::HashMap;

fn main() -> Result<(), serde_yml::Error> {
  let mut map = HashMap::new();
  map.insert("name".to_string(), &"John");
  map.insert("age".to_string(), &"30");

  let yaml = serde_yml::to_string(&map)?;
  println!("Serialized YAML: {}", yaml);

  let deserialized_map: HashMap<String, serde_yml::Value> = serde_yml::from_str(&yaml)?;
   println!("Deserialized map: {:?}", deserialized_map);

   Ok(())
}
```

This example demonstrates how to serialize and deserialize a `HashMap` to and from YAML using the `serde_yml` crate.

![divider][divider]

### Serializing and Deserializing Custom Structs

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

fn main() -> Result<(), serde_yml::Error> {
  let person = Person {
      name: "Alice".to_string(),
      age: 25,
      city: "New York".to_string(),
  };

  let yaml = serde_yml::to_string(&person)?;
  println!("Serialized YAML: {}", yaml);

  let deserialized_person: Person = serde_yml::from_str(&yaml)?;
  println!("Deserialized person: {:?}", deserialized_person);

  Ok(())
}
```

This example demonstrates how to serialize and deserialize a custom struct `Person` to and from YAML using the `serde_yml` crate.

![divider][divider]

### Using Serde derive

It can also be used with Serde's derive macros to handle structs and enums
defined in your program.

Structs serialize in the obvious way:

```rust
# use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() -> Result<(), serde_yml::Error> {
    let point = Point { x: 1.0, y: 2.0 };

    let yaml = serde_yml::to_string(&point)?;
    assert_eq!(yaml, "x: 1.0\n'y': 2.0\n");

    let deserialized_point: Point = serde_yml::from_str(&yaml)?;
    assert_eq!(point, deserialized_point);
    Ok(())
}
```

Enums serialize using YAML's `!tag` syntax to identify the variant name.

```rust
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Enum {
    Unit,
    Newtype(usize),
    Tuple(usize, usize, usize),
    Struct { x: f64, y: f64 },
}

fn main() -> Result<(), serde_yml::Error> {
    let yaml = "
        - !Newtype 1
        - !Tuple [0, 0, 0]
        - !Struct {x: 1.0, y: 2.0}
    ";
    let values: Vec<Enum> = serde_yml::from_str(yaml).unwrap();
    assert_eq!(values[0], Enum::Newtype(1));
    assert_eq!(values[1], Enum::Tuple(0, 0, 0));
    assert_eq!(values[2], Enum::Struct { x: 1.0, y: 2.0 });

    // The last two in YAML's block style instead:
    let yaml = "
        - !Tuple
        - 0
        - 0
        - 0
        - !Struct
        x: 1.0
        'y': 2.0
    ";
    let values: Vec<Enum> = serde_yml::from_str(yaml).unwrap();
    assert_eq!(values[0], Enum::Tuple(0, 0, 0));
    assert_eq!(values[1], Enum::Struct { x: 1.0, y: 2.0 });

    // Variants with no data can be written using !Tag or just the string name.
    let yaml = "
        - Unit  # serialization produces this one
        - !Unit
    ";
    let values: Vec<Enum> = serde_yml::from_str(yaml).unwrap();
    assert_eq!(values[0], Enum::Unit);
    assert_eq!(values[1], Enum::Unit);

    Ok(())
}
```

This example demonstrates how to use Serde's derive macros to automatically implement the `Serialize` and `Deserialize` traits for a struct `Point`, and then serialize and deserialize it to and from YAML using the `serde_yml` crate.

![divider][divider]

### Serializing and Deserializing Enums with Custom Serialization and Deserialization

```rust
use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum MyEnum {
    Variant1(String),
    Variant2 { field: i32 },
}

#[derive(PartialEq, Debug)]
struct MyStruct {
    field: MyEnum,
}

// Custom Serialize and Deserialize implementations for MyStruct
// ...

fn main() {
    let input = MyStruct {
        field: MyEnum::Variant2 { field: 42 },
    };

    let yaml = serde_yml::to_string(&input).unwrap();
    println!("\n✅ Serialized YAML:\n{}", yaml);

    let output: MyStruct = serde_yml::from_str(&yaml).unwrap();
    println!("\n✅ Deserialized YAML:\n{:#?}", output);

    assert_eq!(input, output);
}
```

This example demonstrates how to use custom `Serialize` and `Deserialize` implementations for a struct containing an enum field, and how to leverage `serde_yml` to serialize and deserialize the struct to and from YAML.

![divider][divider]

### Serializing and Deserializing Optional Enums

```rust
use serde::{Deserialize, Serialize};
use serde_yml::with::singleton_map_optional;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum OptionalEnum {
    Variant1(String),
    Variant2 { field: i32 },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct OptionalStruct {
    #[serde(with = "singleton_map_optional")]
    field: Option<OptionalEnum>,
}

// ...

fn main() {
    let input = OptionalStruct {
        field: Some(OptionalEnum::Variant2 { field: 42 }),
    };

    let yaml = serde_yml::to_string(&input).unwrap();
    println!("\n✅ Serialized YAML:\n{}", yaml);

    let output: OptionalStruct = serde_yml::from_str(&yaml).unwrap();
    println!("\n✅ Deserialized YAML:\n{:#?}", output);

    assert_eq!(input, output);
}
```

This example demonstrates how to use the `singleton_map_optional` attribute to serialize and deserialize an `Option<Enum>` field as a single YAML mapping entry with the key being the enum variant name.

![divider][divider]

### Serializing and Deserializing Nested Enums

```rust
use serde::{Deserialize, Serialize};
use serde_yml::with::singleton_map_recursive;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum NestedEnum {
    Variant1(String),
    Variant2(Option<InnerEnum>),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum InnerEnum {
    Inner1(i32),
    Inner2(i32),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct NestedStruct {
    #[serde(with = "singleton_map_recursive")]
    field: NestedEnum,
}

// ...

fn main() {
    let input = NestedStruct {
        field: NestedEnum::Variant2(Some(InnerEnum::Inner2(42))),
    };

    let yaml = serde_yml::to_string(&input).unwrap();
    println!("\n✅ Serialized YAML:\n{}", yaml);

    let output: NestedStruct = serde_yml::from_str(&yaml).unwrap();
    println!("\n✅ Deserialized YAML:\n{:#?}", output);

    assert_eq!(input, output);
}
```

This example demonstrates how to use the `singleton_map_recursive` attribute to serialize and deserialize a nested enum structure where one of the enum variants contains an optional inner enum.

![divider][divider]

### Serializing and Deserializing Enums with `singleton_map_recursive`

```rust
use serde::{Deserialize, Serialize};
use serde_yml::with::singleton_map_recursive;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum MyEnum {
    Variant1(String),
    Variant2 { field: i32 },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct MyStruct {
    #[serde(with = "singleton_map_recursive")]
    field: MyEnum,
}

// ...

fn main() {
    let input = MyStruct {
        field: MyEnum::Variant2 { field: 42 },
    };

    let yaml = serde_yml::to_string(&input).unwrap();
    println!("\n✅ Serialized YAML:\n{}", yaml);

    let output: MyStruct = serde_yml::from_str(&yaml).unwrap();
    println!("\n✅ Deserialized YAML:\n{:#?}", output);

    assert_eq!(input, output);
}
```

This example demonstrates how to use the `singleton_map_recursive` attribute to serialize and deserialize an enum field as a single YAML mapping entry with the key being the enum variant name.

![divider][divider]

### Serializing and Deserializing Enums with `singleton_map_with` and Custom Serialization

```rust
use serde::{Deserialize, Serialize};
use serde_yml::with::singleton_map_with;

fn custom_serialize<T, S>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: serde::Serializer,
{
    // Custom serialization logic
    singleton_map_with::serialize(value, serializer)
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum MyEnum {
    Variant1(String),
    Variant2 { field: i32 },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct MyStruct {
    #[serde(
        serialize_with = "custom_serialize",
        deserialize_with = "singleton_map_with::deserialize"
    )]
    field: MyEnum,
}

fn main() {
    let input = MyStruct {
        field: MyEnum::Variant2 { field: 42 },
    };
    let yaml = serde_yml::to_string(&input).unwrap();
    println!("\n✅ Serialized YAML:\n{}", yaml);

    let output: MyStruct = serde_yml::from_str(&yaml).unwrap();
    println!("\n✅ Deserialized YAML:\n{:#?}", output);
    assert_eq!(input, output);
}
```

This example demonstrates how to use the `singleton_map_with` attribute in combination with a custom serialization function (`custom_serialize`) to serialize and deserialize an enum field (`MyEnum`) within a struct (`MyStruct`).

The `custom_serialize` function is used for serialization, while the `singleton_map_with::deserialize` function is used for deserialization. This allows for additional customization of the serialization process while still leveraging the singleton_map_with attribute for deserialization.

![divider][divider]

### Serializing and Deserializing Enums with `singleton_map_with`

```rust
use serde::{Deserialize, Serialize};
use serde_yml::with::singleton_map_with;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum MyEnum {
    Variant1(String),
    Variant2 { field: i32 },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct MyStruct {
    #[serde(with = "singleton_map_with")]
    field: MyEnum,
}

fn main() {
    let input = MyStruct {
        field: MyEnum::Variant2 { field: 42 },
    };
    let yaml = serde_yml::to_string(&input).unwrap();
    println!("\n✅ Serialized YAML:\n{}", yaml);

    let output: MyStruct = serde_yml::from_str(&yaml).unwrap();
    println!("\n✅ Deserialized YAML:\n{:#?}", output);
    assert_eq!(input, output);
}
```

This example demonstrates how to use the `singleton_map_with` attribute to serialize and deserialize an enum field (`MyEnum`) within a struct (`MyStruct`). The `singleton_map_with` attribute allows for additional customization of the serialization and deserialization process through the use of helper functions.

![divider][divider]

## Best Practices and Common Pitfalls

- When serializing large datasets, consider using `serde_yml::to_writer` to write the YAML output directly to a file or a writer instead of keeping the entire serialized string in memory.
- Be cautious when deserializing untrusted YAML input, as it may contain unexpected or malicious data. Always validate and handle the deserialized data appropriately.
- When working with custom structs or enums, ensure that they implement the necessary Serde traits (`Serialize` and `Deserialize`) for proper serialization and deserialization.
- If you encounter any issues or have questions, refer to the library's documentation and examples for guidance. If the problem persists, consider opening an issue on the library's GitHub repository.

## Installation

To use Serde YML in your Rust project, add the following to your `Cargo.toml` file:

```shell
[dependencies]
serde_yml = "0.0.5"
```

## Semantic Versioning Policy

For transparency into our release cycle and in striving to maintain
backward compatibility, `serde_yml` follows [semantic versioning][07].

## License

The project is licensed under the terms of both the MIT license and the
Apache License (Version 2.0).

- [Apache License, Version 2.0][02]
- [MIT license][03]

## Contribution

We welcome all people who want to contribute. Please see the
[contributing instructions][05] for more information.

Contributions in any form (issues, pull requests, etc.) to this project
must adhere to the [Rust's Code of Conduct][12].

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Acknowledgements

A big thank you to all the awesome contributors of [serde_yml][06] for their help
and support. A special thank you goes to [David Tolnay][16] and his work on
[Serde YAML][17] for inspiring this project.

[00]: https://serdeyml.com "Serde YML"
[01]: https://github.com/serde-rs/serde "Serde"
[02]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
[03]: http://opensource.org/licenses/MIT "MIT license"
[04]: https://github.com/sebastienrousseau/serde_yml/issues "Issues"
[05]: https://github.com/sebastienrousseau/serde_yml/blob/main/CONTRIBUTING.md "Contributing"
[06]: https://github.com/sebastienrousseau/serde_yml/graphs/contributors "Contributors"
[07]: http://semver.org/ "Semantic Versioning"
[08]: https://crates.io/crates/serde_yml "Serde YML on crates.io"
[09]: https://docs.rs/serde_yml "Serde YML on docs.rs"
[10]: https://lib.rs/crates/serde_yml "Serde YML on lib.rs"
[12]: https://www.rust-lang.org/policies/code-of-conduct "Rust's Code of Conduct"
[13]: https://www.rust-lang.org "The Rust Programming Language"
[14]: https://codecov.io/gh/sebastienrousseau/serde_yml "Codecov"
[16]: https://github.com/dtolnay "David Tolnay"
[17]: https://github.com/dtolnay/serde-yaml "Serde YAML"

[banner]: https://kura.pro/serde_yml/images/titles/title-serde_yml.svg "Serde YML Banner"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/serde_yml?style=for-the-badge&token=Q9KJ6XXL67 "Codecov Badge"
[crates-badge]: https://img.shields.io/crates/v/serde_yml.svg?style=for-the-badge "Crates.io Badge"
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/serde_yml.svg?style=for-the-badge "Docs.rs Badge"
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.5-orange.svg?style=for-the-badge "Lib.rs Badge"
[license-badge]: https://img.shields.io/crates/l/serde_yml.svg?style=for-the-badge "License Badge"
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust "Made With Rust Badge"
