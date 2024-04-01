// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

use crate::Value;
use crate::partialeq_numeric;

impl PartialEq<str> for Value {
    /// Compare `str` with YAML value
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_yml::Value;
    /// assert!(Value::String("lorem".into()) == *"lorem");
    /// ```
    fn eq(&self, other: &str) -> bool {
        self.as_str().map_or(false, |s| s == other)
    }
}

impl PartialEq<&str> for Value {
    /// Compare `&str` with YAML value
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_yml::Value;
    /// assert!(Value::String("lorem".into()) == "lorem");
    /// ```
    fn eq(&self, other: &&str) -> bool {
        self.as_str().map_or(false, |s| s == *other)
    }
}

impl PartialEq<String> for Value {
    /// Compare YAML value with String
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_yml::Value;
    /// assert!(Value::String("lorem".into()) == "lorem".to_string());
    /// ```
    fn eq(&self, other: &String) -> bool {
        self.as_str().map_or(false, |s| s == other)
    }
}

impl PartialEq<bool> for Value {
    /// Compare YAML value with bool
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_yml::Value;
    /// assert!(Value::Bool(true) == true);
    /// ```
    fn eq(&self, other: &bool) -> bool {
        self.as_bool().map_or(false, |b| b == *other)
    }
}

fn compare_numeric<T, U>(i: T, other: U) -> bool
where
    T: PartialEq<U>,
    U: Into<T>,
{
    i == other
}

partialeq_numeric! {
    [i8 i16 i32 i64 isize], as_i64, i64
    [u8 u16 u32 u64 usize], as_u64, u64
    [f32 f64], as_f64, f64
}
