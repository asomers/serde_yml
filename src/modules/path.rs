// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

use std::fmt::{self, Display};

/// `Path` represents the path to the current value in the input, like `dependencies.serde.typo1`.
///
/// The `Path` enum provides a way to represent different types of paths in a YAML-like structure.
/// It can be used to track the location of values within the input and provide meaningful error messages.
///
/// # Variants
///
/// - `Root`: Represents the root path.
/// - `Seq`: Represents a sequence (array) path with a reference to the parent path and an index.
/// - `Map`: Represents a map (object) path with a reference to the parent path and a key.
/// - `Alias`: Represents an alias path with a reference to the parent path.
/// - `Unknown`: Represents an unknown path with a reference to the parent path.
#[derive(Copy, Clone, Debug)]
pub enum Path<'a> {
    /// Represents the root path.
    Root,
    /// Represents a sequence (array) path with a reference to the parent path and an index.
    Seq {
        /// The parent path.
        parent: &'a Path<'a>,
        /// The index within the sequence.
        index: usize,
    },
    /// Represents a map (object) path with a reference to the parent path and a key.
    Map {
        /// The parent path.
        parent: &'a Path<'a>,
        /// The key within the map.
        key: &'a str,
    },
    /// Represents an alias path with a reference to the parent path.
    Alias {
        /// The parent path.
        parent: &'a Path<'a>,
    },
    /// Represents an unknown path with a reference to the parent path.
    Unknown {
        /// The parent path.
        parent: &'a Path<'a>,
    },
}

impl Display for Path<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        /// `Parent` is a helper struct used to format the parent path.
        ///
        /// It implements the `Display` trait to recursively format the parent path.
        struct Parent<'a>(&'a Path<'a>);

        impl Display for Parent<'_> {
            fn fmt(
                &self,
                formatter: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                match self.0 {
                    Path::Root => Ok(()),
                    path => write!(formatter, "{}.", path),
                }
            }
        }

        match self {
            Path::Root => formatter.write_str("."),
            Path::Seq { parent, index } => {
                write!(formatter, r"{}\[{}\]", Parent(parent), index)
            }
            Path::Map { parent, key } => {
                write!(formatter, "{}{}", Parent(parent), key)
            }
            Path::Alias { parent } => {
                write!(formatter, "{}", Parent(parent))
            }
            Path::Unknown { parent } => {
                write!(formatter, "{}?", Parent(parent))
            }
        }
    }
}
