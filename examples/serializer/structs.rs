// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

//!
//! Example for serializing structs with the YAML serializer.
//!
//! This example demonstrates how to serialize nested structs into YAML format
//! using the `Serializer` provided by the `serde_yml` crate.
//!

use serde::Serialize;
use serde_yml::{to_string, Result};

#[derive(Serialize)]
struct Address {
    street: String,
    city: String,
    country: String,
}

#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
    address: Address,
}

pub(crate) fn main() -> Result<()> {
    // Print a message to indicate the file being executed.
    println!("\n❯ Executing examples/serializer/structs.rs");

    let user = User {
        name: "Alice".to_string(),
        age: 25,
        address: Address {
            street: "123 Main St".to_string(),
            city: "Anytown".to_string(),
            country: "USA".to_string(),
        },
    };

    let yaml = to_string(&user)?;
    println!("\n✅ User serialized to YAML:\n{}", yaml);

    Ok(())
}
