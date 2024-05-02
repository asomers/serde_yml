// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

//!
//! Example for error handling and serialization of custom error types with the YAML serializer.
//!
//! This example demonstrates how to handle and serialize custom error types
//! with the YAML serializer provided by the `serde_yml` crate.
//!

use serde::Serialize;
use serde_yml::{to_string, Result};
use std::fmt;

#[derive(Serialize, Debug)]
struct CustomError {
    message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomError: {}", self.message)
    }
}

impl std::error::Error for CustomError {}

#[derive(Serialize)]
struct ErrorWrapper {
    error: Option<CustomError>,
}

pub(crate) fn main() -> Result<()> {
    // Print a message to indicate the file being executed.
    println!("\n❯ Executing examples/serializer/error_handling.rs");

    let error = CustomError {
        message: "Something went wrong".to_string(),
    };

    let wrapper = ErrorWrapper { error: Some(error) };

    let yaml = to_string(&wrapper)?;
    println!("\n✅ Custom error type serialized to YAML:\n{}", yaml);

    Ok(())
}
