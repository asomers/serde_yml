// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

use serde_yml::{de::Progress, loader::Loader};

pub(crate) fn main() {
    // Print a message to indicate the file being executed.
    println!("\n❯ Executing examples/loader/io_errors.rs");

    let faulty_reader = std::io::Cursor::new(b"---\n- key: value\n");
    let progress = Progress::Read(Box::new(faulty_reader));

    match Loader::new(progress) {
        Ok(_) => println!("\n✅ Loader created successfully"),
        Err(e) => println!("Failed to create loader: {}", e),
    }
}
