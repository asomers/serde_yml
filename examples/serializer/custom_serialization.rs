// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

//!
//! Example for custom serialization implementations with the YAML serializer.
//!
//! This example demonstrates how to use custom serialization implementations
//! with the YAML serializer provided by the `serde_yml` crate.
//!

use serde::Serialize;
use serde_yml::{to_string, Result};

struct Point {
    x: i32,
    y: i32,
}

impl Serialize for Point {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("({}, {})", self.x, self.y))
    }
}

pub(crate) fn main() -> Result<()> {
    // Print a message to indicate the file being executed.
    println!(
        "\n❯ Executing examples/serializer/custom_serialization.rs"
    );

    let point = Point { x: 3, y: 7 };
    let yaml = to_string(&point)?;
    println!(
        "\n✅ Point serialized with custom implementation:\n{}",
        yaml
    );

    Ok(())
}
