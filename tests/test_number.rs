// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

#[cfg(test)]
mod tests {
    use serde_yml::Number;
    use std::{
        cmp::Ordering,
        hash::{DefaultHasher, Hash, Hasher},
        str::FromStr,
    };

    // Tests for Number::as_f64 method
    #[test]
    fn test_as_f64() {
        let number = Number::from(42);
        assert_eq!(number.as_f64(), Some(42.0));

        let number = Number::from(-42);
        assert_eq!(number.as_f64(), Some(-42.0));

        let number = Number::from(3.14);
        assert_eq!(number.as_f64(), Some(3.14));
    }

    // Tests for Number::as_i64 method
    #[test]
    fn test_as_i64() {
        let number = Number::from(42);
        assert_eq!(number.as_i64(), Some(42));

        let number = Number::from(-42);
        assert_eq!(number.as_i64(), Some(-42));

        let number = Number::from(3.14);
        assert_eq!(number.as_i64(), None);
    }

    // Tests for Number::as_u64 method
    #[test]
    fn test_as_u64() {
        let number = Number::from(42);
        assert_eq!(number.as_u64(), Some(42));

        let number = Number::from(-42);
        assert_eq!(number.as_u64(), None);

        let number = Number::from(3.14);
        assert_eq!(number.as_u64(), None);
    }

    // Tests for Number::display method
    #[test]
    fn test_display() {
        let number = Number::from(42);
        assert_eq!(number.to_string(), "42");

        let number = Number::from(-42);
        assert_eq!(number.to_string(), "-42");

        let number = Number::from(f64::NAN);
        assert_eq!(number.to_string(), ".nan");

        let number = Number::from(f64::INFINITY);
        assert_eq!(number.to_string(), ".inf");

        let number = Number::from(-f64::INFINITY);
        assert_eq!(number.to_string(), "-.inf");

        let number = Number::from(3.14);
        assert_eq!(number.to_string(), "3.14");
    }

    // Tests for Number::from_str method
    #[test]
    fn test_from_str() {
        let number = Number::from_str("42").unwrap();
        assert_eq!(number, Number::from(42));

        let number = Number::from_str("-42").unwrap();
        assert_eq!(number, Number::from(-42));

        let number = Number::from_str("3.14").unwrap();
        assert_eq!(number, Number::from(3.14));

        let result = Number::from_str("invalid");
        assert!(result.is_err());
    }

    // Tests for Number::is_f64 method
    #[test]
    fn test_is_f64() {
        let number = Number::from(42);
        assert!(!number.is_f64());

        let number = Number::from(-42);
        assert!(!number.is_f64());

        let number = Number::from(3.14);
        assert!(number.is_f64());
    }

    // Tests for Number::is_i64 method
    #[test]
    fn test_is_i64() {
        let number = Number::from(42);
        assert!(number.is_i64());

        let number = Number::from(-42);
        assert!(number.is_i64());

        let number = Number::from(3.14);
        assert!(!number.is_i64());
    }

    // Tests for Number::is_infinite method
    #[test]
    fn test_is_infinite() {
        let number = Number::from(f64::INFINITY);
        assert!(number.is_infinite());

        let number = Number::from(-f64::INFINITY);
        assert!(number.is_infinite());

        let number = Number::from(42);
        assert!(!number.is_infinite());

        let number = Number::from(3.14);
        assert!(!number.is_infinite());
    }

    // Tests for Number::is_nan method
    #[test]
    fn test_is_nan() {
        let number = Number::from(f64::NAN);
        assert!(number.is_nan());

        let number = Number::from(42);
        assert!(!number.is_nan());

        let number = Number::from(3.14);
        assert!(!number.is_nan());
    }

    // Tests for Number::is_u64 method
    #[test]
    fn test_is_u64() {
        let number = Number::from(42);
        assert!(number.is_u64());

        let number = Number::from(-42);
        assert!(!number.is_u64());

        let number = Number::from(3.14);
        assert!(!number.is_u64());
    }

    // Tests for PartialEq implementation
    #[test]
    fn test_partial_eq() {
        let number1 = Number::from(42);
        let number2 = Number::from(42);
        assert_eq!(number1, number2);

        let number1 = Number::from(-42);
        let number2 = Number::from(-42);
        assert_eq!(number1, number2);

        let number1 = Number::from(3.14);
        let number2 = Number::from(3.14);
        assert_eq!(number1, number2);

        let number1 = Number::from(42);
        let number2 = Number::from(-42);
        assert_ne!(number1, number2);

        let number1 = Number::from(42);
        let number2 = Number::from(3.14);
        assert_ne!(number1, number2);
    }

    // Tests for PartialOrd implementation
    #[test]
    fn test_partial_ord() {
        let number1 = Number::from(42);
        let number2 = Number::from(42);
        assert_eq!(
            number1.partial_cmp(&number2),
            Some(Ordering::Equal)
        );

        let number1 = Number::from(-42);
        let number2 = Number::from(42);
        assert_eq!(number1.partial_cmp(&number2), Some(Ordering::Less));

        let number1 = Number::from(42);
        let number2 = Number::from(-42);
        assert_eq!(
            number1.partial_cmp(&number2),
            Some(Ordering::Greater)
        );

        let number1 = Number::from(3.14);
        let number2 = Number::from(3.14);
        assert_eq!(
            number1.partial_cmp(&number2),
            Some(Ordering::Equal)
        );

        let number1 = Number::from(3.14);
        let number2 = Number::from(2.71);
        assert_eq!(
            number1.partial_cmp(&number2),
            Some(Ordering::Greater)
        );

        let number1 = Number::from(f64::NAN);
        let number2 = Number::from(f64::NAN);
        assert_eq!(
            number1.partial_cmp(&number2),
            Some(Ordering::Equal)
        );
    }

    // Tests for Hash implementation
    #[test]
    fn test_hash() {
        let mut hasher = DefaultHasher::new();
        let number = Number::from(42);
        number.hash(&mut hasher);
        let hash1 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        let number = Number::from(42);
        number.hash(&mut hasher);
        let hash2 = hasher.finish();

        assert_eq!(hash1, hash2);
    }

    // Tests for serde serialization and deserialization
    #[test]
    fn test_ser_de() {
        let number = Number::from(42);
        let serialized = serde_yml::to_string(&number).unwrap();
        let deserialized: Number = serde_yml::from_str(&serialized).unwrap();
        assert_eq!(number, deserialized);

        let number = Number::from(-42);
        let serialized = serde_yml::to_string(&number).unwrap();
        let deserialized: Number = serde_yml::from_str(&serialized).unwrap();
        assert_eq!(number, deserialized);

        let number = Number::from(3.14);
        let serialized = serde_yml::to_string(&number).unwrap();
        let deserialized: Number = serde_yml::from_str(&serialized).unwrap();
        assert_eq!(number, deserialized);
    }
}
