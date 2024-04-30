// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

//! # Serde YML
//!
//! [![Serde YML Logo](https://kura.pro/serde_yml/images/banners/banner-serde_yml.svg)](https://serde_yml.one "Serde YML: Seamless YAML Serialization for Rust")
//!
//! ## Seamless YAML Serialization for [Rust][03].
//!
//! [![Crates.io](https://img.shields.io/crates/v/serde_yml.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/serde_yml "Crates.io")
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.5-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/serde_yml "Lib.rs")
//! [![License](https://img.shields.io/crates/l/serde_yml.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](https://opensource.org/license/apache-2-0/ "MIT or Apache License, Version 2.0")
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org "Rust")
//!
//! ## Overview
//!
//! [Serde YML][00] is a Rust library that simplifies YAML serialization and deserialization using the popular [Serde][01] framework. It provides a convenient and efficient way to convert Rust data structures to YAML format and vice versa.
//!
//!## Credits and Acknowledgements
//!
//!This new library draws inspiration from the excellent work done by [David Tolnay][04] and the maintainers of the [serde-yaml][05] library. While Serde YML started as a fork of serde-yaml, it has now evolved into a separate library with its own goals and direction in mind.
//!
//!If you are currently using serde-yaml in your projects, we recommend carefully evaluating your requirements and considering the stability and maturity of the original library before migrating to Serde YML.
//!
//!Finally, I would like to express my sincere gratitude to [David Tolnay][04] and the [serde-yaml][05] team for their valuable contributions to the Rust community and for inspiring this project.
//!
//! ## Features
//!
//! - Serialization and deserialization of Rust data structures to/from YAML format.
//! - Support for custom structs and enums using Serde's derive macros.
//! - Handling of YAML's `!tag` syntax for representing enum variants
//! - Direct access to YAML values through the `Value` type and related types like `Mapping` and `Sequence`.
//! - Comprehensive error handling with `Error`, `Location`, and `Result` types
//! - Serialization to YAML using `to_string` and `to_writer` functions.
//! - Deserialization from YAML using `from_str`, `from_slice`, and `from_reader` functions.
//! - Customizable serialization and deserialization behaviour using Serde's `#[serde(with = ...)]` attribute.
//! - Support for serializing/deserializing enums using a YAML map with a single key-value pair through the `singleton_map` module.
//! - Recursive application of `singleton_map` serialization/deserialization to all enums within a data structure using the `singleton_map_recursive` module.
//! - Well-documented with examples and usage guidelines.
//!
//! ### Rust Version Compatibility
//!
//! This library is compatible with Rust 1.60 and above.
//!
//! ## Usage
//!
//! Serde YML offers a straightforward and intuitive API for working with YAML data in Rust. Here's a quick example of how to serialize and deserialize a Rust type:
//!
//! ```shell
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Point {
//!     x: f64,
//!     y: f64,
//! }
//!
//! fn main() -> Result<(), serde_yml::Error> {
//!     let point = Point { x: 1.0, y: 2.0 };
//!
//!     // Serialize to YAML
//!     let yaml = serde_yml::to_string(&point)?;
//!     assert_eq!(yaml, "x: 1.0\ny: 2.0\n");
//!
//!     // Deserialize from YAML
//!     let deserialized_point: Point = serde_yml::from_str(&yaml)?;
//!     assert_eq!(point, deserialized_point);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Examples
//!
//! ### Serializing and Deserializing a HashMap
//!
//! ```
//! use std::collections::HashMap;
//!
//! fn main() -> Result<(), serde_yml::Error> {
//!     let mut map = HashMap::new();
//!     map.insert("name".to_string(), &"John");
//!     map.insert("age".to_string(), &"30");
//!
//!     let yaml = serde_yml::to_string(&map)?;
//!     println!("Serialized YAML: {}", yaml);
//!
//!     let deserialized_map: HashMap<String, serde_yml::Value> = serde_yml::from_str(&yaml)?;
//!     println!("Deserialized map: {:?}", deserialized_map);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Serializing and Deserializing Custom Structs
//!
//! ```
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! struct Person {
//!     name: String,
//!     age: u32,
//!     city: String,
//! }
//!
//! fn main() -> Result<(), serde_yml::Error> {
//!     let person = Person {
//!         name: "Alice".to_string(),
//!         age: 25,
//!         city: "New York".to_string(),
//!     };
//!
//!     let yaml = serde_yml::to_string(&person)?;
//!     println!("Serialized YAML: {}", yaml);
//!
//!     let deserialized_person: Person = serde_yml::from_str(&yaml)?;
//!     println!("Deserialized person: {:?}", deserialized_person);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Using Serde derive
//!
//! It can also be used with Serde's derive macros to handle structs and enums
//! defined in your program.
//!
//! Structs serialize in the obvious way:
//!
//! ```
//! # use serde_derive::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! struct Point {
//!     x: f64,
//!     y: f64,
//! }
//!
//! fn main() -> Result<(), serde_yml::Error> {
//!     let point = Point { x: 1.0, y: 2.0 };
//!
//!     let yaml = serde_yml::to_string(&point)?;
//!     assert_eq!(yaml, "x: 1.0\n'y': 2.0\n");
//!
//!     let deserialized_point: Point = serde_yml::from_str(&yaml)?;
//!     assert_eq!(point, deserialized_point);
//!     Ok(())
//! }
//! ```
//!
//! Enums serialize using YAML's `!tag` syntax to identify the variant name.
//!
//! ```
//! use serde_derive::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! enum Enum {
//!     Unit,
//!     Newtype(usize),
//!     Tuple(usize, usize, usize),
//!     Struct { x: f64, y: f64 },
//! }
//!
//! fn main() -> Result<(), serde_yml::Error> {
//!     let yaml = "
//!         - !Newtype 1
//!         - !Tuple [0, 0, 0]
//!         - !Struct {x: 1.0, y: 2.0}
//!     ";
//!     let values: Vec<Enum> = serde_yml::from_str(yaml).unwrap();
//!     assert_eq!(values[0], Enum::Newtype(1));
//!     assert_eq!(values[1], Enum::Tuple(0, 0, 0));
//!     assert_eq!(values[2], Enum::Struct { x: 1.0, y: 2.0 });
//!
//!     // The last two in YAML's block style instead:
//!     let yaml = "
//!         - !Tuple
//!           - 0
//!           - 0
//!           - 0
//!         - !Struct
//!           x: 1.0
//!           'y': 2.0
//!     ";
//!     let values: Vec<Enum> = serde_yml::from_str(yaml).unwrap();
//!     assert_eq!(values[0], Enum::Tuple(0, 0, 0));
//!     assert_eq!(values[1], Enum::Struct { x: 1.0, y: 2.0 });
//!
//!     // Variants with no data can be written using !Tag or just the string name.
//!     let yaml = "
//!         - Unit  # serialization produces this one
//!         - !Unit
//!     ";
//!     let values: Vec<Enum> = serde_yml::from_str(yaml).unwrap();
//!     assert_eq!(values[0], Enum::Unit);
//!     assert_eq!(values[1], Enum::Unit);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Best Practices and Common Pitfalls
//!
//! - When serializing large datasets, consider using `serde_yml::to_writer` to write the YAML output directly to a file or a writer instead of keeping the entire serialized string in memory.
//! - Be cautious when deserializing untrusted YAML input, as it may contain unexpected or malicious data. Always validate and handle the deserialized data appropriately.
//! - When working with custom structs or enums, ensure that they implement the necessary Serde traits (`Serialize` and `Deserialize`) for proper serialization and deserialization.
//! - If you encounter any issues or have questions, refer to the library's documentation and examples for guidance. If the problem persists, consider opening an issue on the library's GitHub repository.
//!
//! [00]: https://serdeyml.com "Serde YML"
//! [01]: https://github.com/serde-rs/serde
//! [02]: https://yaml.org/
//! [03]: https://www.rust-lang.org/ "Rust"
//! [04]: https://github.com/dtolnay "David Tolnay"
//! [05]: https://github.com/dtolnay/serde-yaml "Serde YAML"
//!
#![deny(missing_docs)]
#![doc(
    html_favicon_url = "https://kura.pro/serde_yml/images/favicon.ico",
    html_logo_url = "https://kura.pro/serde_yml/images/logos/serde_yml.svg",
    html_root_url = "https://docs.rs/serde_yml"
)]
#![crate_name = "serde_yml"]
#![crate_type = "lib"]

use crate::utilities::generator_uuid::generate_unique_string;
use dtt::DateTime;
use rlg::{log_format::LogFormat, log_level::LogLevel, macro_log};
use std::{fs::File, io::Write};

pub use crate::de::{from_reader, from_slice, from_str, Deserializer};
pub use crate::modules::error::{Error, Location, Result};
pub use crate::ser::{to_string, to_writer, Serializer};
#[doc(inline)]
pub use crate::value::{
    from_value, to_value, Index, Number, Sequence, Value,
};

/// The `generators` module contains functions for generating data.
pub mod generators;
/// The `macros` module contains functions for generating macros.
pub mod macros;
/// The `models` module contains the data models for the library.
pub mod models;
/// The `utilities` module contains utility functions for the library.
pub mod utilities;

#[doc(inline)]
pub use crate::mapping::Mapping;

/// The `de` module contains the library's YAML deserializer.
pub mod de;

/// The `libyml` module contains the library's YAML parser and emitter.
pub mod libyml;

/// The `loader` module contains the `Loader` type for YAML loading.
pub mod loader;

/// The `mapping` module contains the `Mapping` type for YAML mappings.
pub mod mapping;

/// The `modules` module contains the library's modules.
pub mod modules;

mod number;
mod ser;
/// The `value` module contains the `Value` type for YAML values.
pub mod value;
/// The `with` module contains the `With` type for YAML values.
pub mod with;

// Prevent downstream code from implementing the Index trait.
mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for String {}
    impl Sealed for crate::Value {}
    impl<T> Sealed for &T where T: ?Sized + Sealed {}
}

/// Run the Serde YML tool.
pub fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let date = DateTime::new();
    let iso = date.iso_8601;

    // Open the log file for appending
    let mut log_file = File::create("./serde_yml.log")?;

    // Generate ASCII art for the tool's CLI
    let log = macro_log!(
        &generate_unique_string(),
        &iso,
        &LogLevel::INFO,
        "deps",
        "ASCII art generation event started.",
        &LogFormat::CLF
    );
    // Write the log to both the console and the file
    writeln!(log_file, "{}", log)?;

    // Printing the ASCII art to the console
    println!("{}", macro_ascii!("Serde YML"));

    let log = macro_log!(
        &generate_unique_string(),
        &iso,
        &LogLevel::INFO,
        "deps",
        "ASCII art generation event completed.",
        &LogFormat::CLF
    );
    // Write the log to both the console and the file
    writeln!(log_file, "{}", log)?;

    // Check the number of arguments, provide a welcome message if no arguments were passed
    macro_log!(
        &generate_unique_string(),
        &iso,
        &LogLevel::INFO,
        "cli",
        "Welcome to Serde YML! 👋",
        &LogFormat::CLF
    );
    eprintln!("\n\nWelcome to Serde YML! 👋");

    Ok(())
}
