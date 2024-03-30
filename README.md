<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/serde_yml/images/logos/serde_yml.webp"
alt="Serde YML logo" width="261" align="right" />

<!-- markdownlint-enable MD033 MD041 -->
# Serde YML: Seamless YAML Serialization for Rust

Serde YML is a Rust library that simplifies YAML serialization and deserialization using Serde. Effortlessly convert Rust types to YAML and vice versa. Supports custom structs, enums, and error handling.

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

![Banner of Serde YML][banner]

[![Made With Rust][made-with-rust-badge]][13] [![Crates.io][crates-badge]][08] [![Lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][09] [![License][license-badge]][03] [![Codecov][codecov-badge]][14]

• [Website][01] • [Documentation][09] • [Report Bug][04] • [Request Feature][04] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

<!-- markdownlint-enable MD033 -->

![divider][divider]

## Overview

`Serde YML` is a robust Rust library that simplifies the serialization and deserialization of Rust data structures to and from YAML format using the widely-used Serde framework. With Serde YML, you can effortlessly convert your Rust types into YAML strings and vice versa, streamlining the process of storing, transmitting, and manipulating structured data.providing style guides for your library.

## Features

- Serialize Rust data structures to YAML format
- Deserialize YAML data into Rust types
- Support for custom structs and enums using Serde's derive macros
- Handling of YAML's `!tag` syntax for representing enum variants
- Direct access to YAML values through the `Value` type and related types
- Comprehensive error handling with `Error`, `Location`, and `Result` types
- Well-documented with examples and explanations

## Usage

Serde YML offers a straightforward and intuitive API for working with YAML data in Rust. Here's a quick example of how to serialize and deserialize a Rust type:

```shell
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

## Best Practices and Common Pitfalls

- When serializing large datasets, consider using `serde_yml::to_writer` to write the YAML output directly to a file or a writer instead of keeping the entire serialized string in memory.
- Be cautious when deserializing untrusted YAML input, as it may contain unexpected or malicious data. Always validate and handle the deserialized data appropriately.
- When working with custom structs or enums, ensure that they implement the necessary Serde traits (`Serialize` and `Deserialize`) for proper serialization and deserialization.
- If you encounter any issues or have questions, refer to the library's documentation and examples for guidance. If the problem persists, consider opening an issue on the library's GitHub repository.


## Installation

To use Serde YML in your Rust project, add the following to your `Cargo.toml` file:

```shell
[dependencies]
serde_yml = "0.0.3"
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

[01]: https://serdeyml.com "Serde YML"
[02]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
[03]: http://opensource.org/licenses/MIT "MIT license"
[04]: https://github.com/sebastienrousseau/serde_yml/issues "Issues"
[05]: https://github.com/sebastienrousseau/serde_yml/blob/main/CONTRIBUTING.md "Contributing"
[06]: https://github.com/sebastienrousseau/serde_yml/graphs/contributors "Contributors"
[07]: http://semver.org/ "Semantic Versioning"
[08]: https://crates.io/crates/serde_yml "Serde YML on crates.io"
[09]: https://docs.rs/serde_yml "Serde YML on docs.rs"
[10]: https://lib.rs/crates/serde_yml "Serde YML on lib.rs"
[11]: https://github.com/sebastienrousseau/serde_yml/actions "GitHub Actions"
[12]: https://www.rust-lang.org/policies/code-of-conduct "Rust's Code of Conduct"
[13]: https://www.rust-lang.org "The Rust Programming Language"
[14]: https://codecov.io/gh/sebastienrousseau/serde_yml "Codecov"
[15]: https://www.reddit.com/r/rust/ "Rust Reddit"
[16]: https://github.com/dtolnay "David Tolnay"
[17]: https://github.com/dtolnay/serde_yml "Serde YAML"


[banner]: https://kura.pro/serde_yml/images/titles/title-serde_yml.svg "Serde YML Banner"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/serde_yml?style=for-the-badge&token=Q9KJ6XXL67 "Codecov Badge"
[crates-badge]: https://img.shields.io/crates/v/serde_yml.svg?style=for-the-badge "Crates.io Badge"
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/serde_yml.svg?style=for-the-badge "Docs.rs Badge"
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.3-orange.svg?style=for-the-badge "Lib.rs Badge"
[license-badge]: https://img.shields.io/crates/l/serde_yml.svg?style=for-the-badge "License Badge"
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust "Made With Rust Badge"
