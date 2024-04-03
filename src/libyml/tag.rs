// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

use crate::libyml::cstr;
use std::{
    fmt::{self, Debug},
    ops::Deref,
};

/// Represents a tag in a YAML document.
/// A tag specifies the data type or semantic meaning of a value.
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct Tag(pub(in crate::libyml) Box<[u8]>);

impl Tag {
    /// The null tag, representing a null value.
    pub const NULL: &'static str = "tag:yaml.org,2002:null";

    /// The bool tag, representing a boolean value.
    pub const BOOL: &'static str = "tag:yaml.org,2002:bool";

    /// The int tag, representing an integer value.
    pub const INT: &'static str = "tag:yaml.org,2002:int";

    /// The float tag, representing a floating-point value.
    pub const FLOAT: &'static str = "tag:yaml.org,2002:float";

    /// Checks if the tag starts with the given prefix.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix to check against.
    ///
    /// # Returns
    ///
    /// Returns `true` if the tag starts with the given prefix, `false` otherwise.
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.0.starts_with(prefix.as_bytes())
    }
}

impl PartialEq<str> for Tag {
    /// Checks if the tag is equal to the given string.
    ///
    /// # Arguments
    ///
    /// * `other` - The string to compare against.
    ///
    /// # Returns
    ///
    /// Returns `true` if the tag is equal to the given string, `false` otherwise.
    fn eq(&self, other: &str) -> bool {
        self.0 == other.as_bytes().into()
    }
}

impl Deref for Tag {
    type Target = [u8];

    /// Dereferences the tag to its underlying byte slice.
    ///
    /// # Returns
    ///
    /// Returns a reference to the underlying byte slice of the tag.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for Tag {
    /// Formats the tag for debugging purposes.
    ///
    /// # Arguments
    ///
    /// * `formatter` - The formatter to write the debug output to.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the formatting was successful, or an error otherwise.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        cstr::debug_lossy(&self.0, formatter)
    }
}
