// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

//!
//! Example for basic usage of the YAML serializer.
//!
//! This example demonstrates how to serialize a simple struct into YAML format
//! using the `Serializer` provided by the `serde_yml` crate.
//!

use serde::Serialize;
use serde_yml::Serializer;

#[derive(Serialize)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

pub(crate) fn main() {
    // Print a message to indicate the file being executed.
    println!("\n❯ Executing examples/serializer/basic.rs");

    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
        city: "New York".to_string(),
    };

    let mut serializer = Serializer::new(std::io::stdout());
    person.serialize(&mut serializer).unwrap();

    println!("\n✅ Person serialized to YAML.");
}
