// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

//!
//! Example for serializing optional fields and default values with the YAML serializer.
//!
//! This example demonstrates how the serializer handles optional fields and default values
//! when serializing a struct into YAML format using the `Serializer` provided by the `serde_yml` crate.
//!

use serde::Serialize;
use serde_yml::{to_string, Result};

#[derive(Serialize)]
struct User {
    name: String,
    age: Option<u32>,
    #[serde(default)]
    is_active: bool,
}

pub(crate) fn main() -> Result<()> {
    // Print a message to indicate the file being executed.
    println!("\n❯ Executing examples/serializer/optional_and_default.rs");

    let user1 = User {
        name: "John".to_string(),
        age: Some(30),
        is_active: true,
    };

    let yaml = to_string(&user1)?;
    println!("\n✅ User with optional fields serialized to YAML:\n{}", yaml);

    let user2 = User {
        name: "Jane".to_string(),
        age: None,
        is_active: false,
    };

    let yaml = to_string(&user2)?;
    println!("\n✅ User with default values serialized to YAML:\n{}", yaml);

    Ok(())
}
