// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

/// This module contains the `singleton_map` example.
pub(crate) mod singleton_map;

/// This module contains the `singleton_map_recursive`
pub(crate) mod singleton_map_recursive;

/// This module contains the `singleton_map_enum_variants` example.
pub(crate) mod singleton_map_enum_variants;

/// This module contains the `singleton_map_recursive_deep_nesting` example.
pub(crate) mod singleton_map_recursive_deep_nesting;

/// This module contains the `singleton_map_recursive_serialize_deserialize` example.
pub(crate) mod singleton_map_recursive_serialize_deserialize;

/// The main function that runs all the example modules.
pub(crate) fn main() {
    // Run the example module `loader_anchors_and_aliases`.
    singleton_map::main();

    // Run the example module `singleton_map_recursive`.
    singleton_map_recursive::main();

    // Run the example module `singleton_map_enum_variants`.
    singleton_map_enum_variants::main();

    // Run the example module `singleton_map_recursive_deep_nesting`.
    singleton_map_recursive_deep_nesting::main();

    // Run the example module `singleton_map_recursive_serialize_deserialize`.
    singleton_map_recursive_serialize_deserialize::main();
}
