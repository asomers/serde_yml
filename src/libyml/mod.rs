// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

/// C string handling.
mod cstr;
/// YAML emitter.
pub mod emitter;
/// Error handling.
pub mod error;
/// YAML parser.
pub mod parser;
/// Tag directive handling.
pub mod tag;
/// Utility types.
pub mod util;

use self::error::Error;
