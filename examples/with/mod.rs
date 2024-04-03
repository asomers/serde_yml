
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
