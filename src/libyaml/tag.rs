// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

use crate::libyaml::cstr;
use std::fmt::{self, Debug};
use std::ops::Deref;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Tag(pub(in crate::libyaml) Box<[u8]>);

impl Tag {
    pub const NULL: &'static str = "tag:yaml.org,2002:null";
    pub const BOOL: &'static str = "tag:yaml.org,2002:bool";
    pub const INT: &'static str = "tag:yaml.org,2002:int";
    pub const FLOAT: &'static str = "tag:yaml.org,2002:float";

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.0.starts_with(prefix.as_bytes())
    }


}

impl PartialEq<str> for Tag {
    fn eq(&self, other: &str) -> bool {
        self.0 == other.as_bytes().into()
    }
}

impl Deref for Tag {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for Tag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        cstr::debug_lossy(&self.0, formatter)
    }
}
