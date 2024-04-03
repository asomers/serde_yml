// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

//! # Serde YML Examples
//!
//! This crate contains examples that demonstrate the usage of the Serde YML library.
//!
//! The examples are organized into the following modules:
//!
//! - `loader` - Contains the example modules for the `loader` module.
//! - `with` - Contains the example modules for the `with` module.
//!

/// Contains the example modules for the `loader` module.
mod loader;

/// Contains the example modules for the `serializer` module.
mod serializer;

/// Examples for the `with` module.
mod with;

/// The main function that runs all the example modules.
///
/// This function is responsible for running all the example modules.
/// It does this by calling the `main` function of each example module.
///
fn main() {
    // Run the example module `loader`.
    loader::main();

    // Run the example module `serializer`.
    serializer::main();

    // Run the example module `with`.
    with::main();
}
