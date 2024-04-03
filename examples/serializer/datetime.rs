// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

//!
//! Example for serializing RFC 3339 formatted dates and times with the YAML serializer.
//!
//! This example demonstrates how the serializer handles date and time values formatted
//! according to RFC 3339 when serializing a struct into YAML format using the `Serializer`
//! provided by the `serde_yml` crate.
//!

use dtt::DateTime;
use serde::Serialize;
use serde_yml::{to_string, Result};

#[derive(Serialize)]
struct Event {
    name: String,
    timestamp: DateTime,
}

pub(crate) fn main() -> Result<()> {
    // Print a message to indicate the file being executed.
    println!("\n❯ Executing examples/serializer/datetime.rs");

    let event = Event {
        name: "Paris Conference".to_string(),
        timestamp: DateTime::new_with_tz("CEST"),
    };

    let yaml = to_string(&event)?;
    println!("\n✅ Event serialized to YAML:\n{}", yaml);

    Ok(())
}
