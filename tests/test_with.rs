// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_yml::with::*;

    // Define the enum MyEnum
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum MyEnum {
        Unit,
        Newtype(usize),
        Tuple(usize, usize),
        Struct { value: usize },
    }

    // Test serialization and deserialization using singleton_map
    #[test]
    fn test_singleton_map() {
        // Define struct TestStruct for serialization
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestStruct {
            #[serde(with = "singleton_map")]
            field: MyEnum,
        }

        // Test serialization and deserialization for each variant of MyEnum
        // Variant: Unit
        let test_struct = TestStruct {
            field: MyEnum::Unit,
        };
        let yaml = serde_yml::to_string(&test_struct).unwrap();
        assert_eq!(yaml, "field: Unit\n");
        let deserialized: TestStruct =
            serde_yml::from_str(&yaml).unwrap();
        assert_eq!(test_struct, deserialized);

        // Variant: Newtype
        let test_struct = TestStruct {
            field: MyEnum::Newtype(42),
        };
        let yaml = serde_yml::to_string(&test_struct).unwrap();
        assert_eq!(yaml, "field:\n  Newtype: 42\n");
        let deserialized: TestStruct =
            serde_yml::from_str(&yaml).unwrap();
        assert_eq!(test_struct, deserialized);

        // Variant: Tuple
        let test_struct = TestStruct {
            field: MyEnum::Tuple(1, 2),
        };
        let yaml = serde_yml::to_string(&test_struct).unwrap();
        assert_eq!(yaml, "field:\n  Tuple:\n  - 1\n  - 2\n");
        let deserialized: TestStruct =
            serde_yml::from_str(&yaml).unwrap();
        assert_eq!(test_struct, deserialized);

        // Variant: Struct
        let test_struct = TestStruct {
            field: MyEnum::Struct { value: 42 },
        };
        let yaml = serde_yml::to_string(&test_struct).unwrap();
        assert_eq!(yaml, "field:\n  Struct:\n    value: 42\n");
        let deserialized: TestStruct =
            serde_yml::from_str(&yaml).unwrap();
        assert_eq!(test_struct, deserialized);
    }

    // Test serialization and deserialization using singleton_map_optional
    #[test]
    fn test_singleton_map_optional() {
        // Define struct TestStruct for serialization
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestStruct {
            #[serde(with = "singleton_map_optional")]
            field: Option<MyEnum>,
        }

        // Test serialization and deserialization for Some(MyEnum::Unit) and None
        let test_struct = TestStruct {
            field: Some(MyEnum::Unit),
        };
        let yaml = serde_yml::to_string(&test_struct).unwrap();
        assert_eq!(yaml, "field: Unit\n");
        let deserialized: TestStruct =
            serde_yml::from_str(&yaml).unwrap();
        assert_eq!(test_struct, deserialized);

        let test_struct = TestStruct { field: None };
        let yaml = serde_yml::to_string(&test_struct).unwrap();
        assert_eq!(yaml, "field: null\n");
        let deserialized: TestStruct =
            serde_yml::from_str(&yaml).unwrap();
        assert_eq!(test_struct, deserialized);
    }

    // Test serialization and deserialization using singleton_map_with
    #[test]
    fn test_singleton_map_with() {
        // Define struct TestStruct for serialization
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestStruct {
            #[serde(with = "singleton_map_with")]
            field: MyEnum,
        }

        // Test serialization and deserialization for MyEnum::Unit
        let test_struct = TestStruct {
            field: MyEnum::Unit,
        };
        let yaml = serde_yml::to_string(&test_struct).unwrap();
        assert_eq!(yaml, "field: Unit\n");
        let deserialized: TestStruct =
            serde_yml::from_str(&yaml).unwrap();
        assert_eq!(test_struct, deserialized);
    }

    // Test serialization and deserialization using singleton_map_recursive
    #[test]
    fn test_singleton_map_recursive() {
        // Define enum NestedEnum and struct TestStruct for serialization
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum NestedEnum {
            Variant(MyEnum),
        }

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestStruct {
            #[serde(with = "singleton_map_recursive")]
            field: NestedEnum,
        }

        // Test serialization and deserialization for NestedEnum::Variant(MyEnum::Unit)
        let test_struct = TestStruct {
            field: NestedEnum::Variant(MyEnum::Unit),
        };
        let yaml = serde_yml::to_string(&test_struct).unwrap();
        assert_eq!(yaml, "field:\n  Variant: Unit\n");
        let deserialized: TestStruct =
            serde_yml::from_str(&yaml).unwrap();
        assert_eq!(test_struct, deserialized);
    }

    // Test top-level singleton_map_recursive serialization and deserialization
    #[test]
    fn test_singleton_map_recursive_top_level() {
        // Test serialization and deserialization for MyEnum::Unit
        let value = MyEnum::Unit;
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map_recursive::serialize(&value, &mut serializer)
            .unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Unit\n");

        let deserialized: MyEnum =
            singleton_map_recursive::deserialize(
                serde_yml::Deserializer::from_str(&yaml),
            )
            .unwrap();
        assert_eq!(value, deserialized);
    }

    // Test singleton_map serialization
    #[test]
    fn test_singleton_map_serialization() {
        // Test serialization for each variant of MyEnum
        let value = MyEnum::Unit;
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Unit\n");

        let value = MyEnum::Newtype(42);
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Newtype: 42\n");

        let value = MyEnum::Tuple(1, 2);
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Tuple:\n- 1\n- 2\n");

        let value = MyEnum::Struct { value: 42 };
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Struct:\n  value: 42\n");
    }

    // Test singleton_map deserialization
    #[test]
    fn test_singleton_map_deserialization() {
        // Test deserialization for each variant of MyEnum
        let yaml = "Unit\n";
        let deserialized: MyEnum = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        )
        .unwrap();
        assert_eq!(deserialized, MyEnum::Unit);

        let yaml = "Newtype: 42\n";
        let deserialized: MyEnum = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        )
        .unwrap();
        assert_eq!(deserialized, MyEnum::Newtype(42));

        let yaml = "Tuple:\n- 1\n- 2\n";
        let deserialized: MyEnum = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        )
        .unwrap();
        assert_eq!(deserialized, MyEnum::Tuple(1, 2));

        let yaml = "Struct:\n  value: 42\n";
        let deserialized: MyEnum = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        )
        .unwrap();
        assert_eq!(deserialized, MyEnum::Struct { value: 42 });
    }

    // Test singleton_map_optional serialization
    #[test]
    fn test_singleton_map_optional_serialization() {
        // Test serialization for Some(MyEnum::Unit) and None
        let value = Some(MyEnum::Unit);
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map_optional::serialize(&value, &mut serializer)
            .unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Unit\n");

        let value: Option<MyEnum> = None;
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map_optional::serialize(&value, &mut serializer)
            .unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "null\n");
    }

    // Test singleton_map_optional deserialization
    #[test]
    fn test_singleton_map_optional_deserialization() {
        // Test deserialization for Some(MyEnum::Unit) and None
        let yaml = "Unit\n";
        let deserialized: Option<MyEnum> =
            singleton_map_optional::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            )
            .unwrap();
        assert_eq!(deserialized, Some(MyEnum::Unit));

        let yaml = "null\n";
        let deserialized: Option<MyEnum> =
            singleton_map_optional::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            )
            .unwrap();
        assert_eq!(deserialized, None);
    }

    // Test singleton_map_with serialization
    #[test]
    fn test_singleton_map_with_serialization() {
        // Test serialization for MyEnum::Unit
        let value = MyEnum::Unit;
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map_with::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Unit\n");
    }

    // Test singleton_map_with deserialization
    #[test]
    fn test_singleton_map_with_deserialization() {
        // Test deserialization for MyEnum::Unit
        let yaml = "Unit\n";
        let deserialized: MyEnum = singleton_map_with::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        )
        .unwrap();
        assert_eq!(deserialized, MyEnum::Unit);
    }

    // Test singleton_map_recursive serialization
    #[test]
    fn test_singleton_map_recursive_serialization() {
        // Define enum NestedEnum for serialization
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum NestedEnum {
            Variant(MyEnum),
        }

        // Test serialization for NestedEnum::Variant(MyEnum::Unit)
        let value = NestedEnum::Variant(MyEnum::Unit);
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map_recursive::serialize(&value, &mut serializer)
            .unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Variant: Unit\n");
    }

    // Test singleton_map_recursive deserialization
    #[test]
    fn test_singleton_map_recursive_deserialization() {
        // Define enum NestedEnum for deserialization
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum NestedEnum {
            Variant(MyEnum),
        }

        // Test deserialization for NestedEnum::Variant(MyEnum::Unit)
        let yaml = "Variant: Unit\n";
        let deserialized: NestedEnum =
            singleton_map_recursive::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            )
            .unwrap();
        assert_eq!(deserialized, NestedEnum::Variant(MyEnum::Unit));
    }

    // Test top-level singleton_map_recursive serialization
    #[test]
    fn test_singleton_map_recursive_top_level_serialization() {
        // Test serialization for MyEnum::Unit
        let value = MyEnum::Unit;
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map_recursive::serialize(&value, &mut serializer)
            .unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Unit\n");
    }

    // Test top-level singleton_map_recursive deserialization
    #[test]
    fn test_singleton_map_recursive_top_level_deserialization() {
        // Test deserialization for MyEnum::Unit
        let yaml = "Unit\n";
        let deserialized: MyEnum =
            singleton_map_recursive::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            )
            .unwrap();
        assert_eq!(deserialized, MyEnum::Unit);
    }
    // Tests for error handling
    #[test]
    fn test_singleton_map_deserialization_error() {
        // Test deserialization error for invalid YAML input
        let yaml = "InvalidYAML";
        let result: Result<MyEnum, _> = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_singleton_map_missing_field_error() {
        // Test deserialization error for missing field
        let yaml = "MissingField: 42";
        let result: Result<MyEnum, _> = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        );
        assert!(result.is_err());
    }

    // Tests for edge cases
    #[test]
    fn test_empty_enum() {
        // Define an enum with a single variant
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum SingleVariantEnum {
            Variant,
        }

        // Test serialization and deserialization of the single-variant enum
        let value = SingleVariantEnum::Variant;
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Variant\n");

        let deserialized: SingleVariantEnum =
            singleton_map::deserialize(
                serde_yml::Deserializer::from_str(&yaml),
            )
            .unwrap();
        assert_eq!(value, deserialized);
    }
    #[test]
    fn test_generic_enum() {
        // Define an enum with generic type parameters
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum GenericEnum<T> {
            Variant(T),
        }

        // Test serialization and deserialization of the generic enum
        let value = GenericEnum::Variant(42);
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Variant: 42\n");

        let deserialized: GenericEnum<i32> =
            singleton_map::deserialize(
                serde_yml::Deserializer::from_str(&yaml),
            )
            .unwrap();
        assert_eq!(value, deserialized);
    }
}
