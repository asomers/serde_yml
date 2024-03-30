// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

#[cfg(test)]
mod tests {
    use serde_yml::modules::path::Path;

    #[test]
    fn test_path_root() {
        let path = Path::Root;
        assert_eq!(format!("{}", path), ".");
    }

    #[test]
    fn test_path_seq() {
        let root = Path::Root;
        let path = Path::Seq {
            parent: &root,
            index: 42,
        };
        assert_eq!(format!("{}", path), "\\[42\\]");
    }

    #[test]
    fn test_path_map() {
        let root = Path::Root;
        let path = Path::Map {
            parent: &root,
            key: "key",
        };
        assert_eq!(format!("{}", path), "key");
    }

    #[test]
    fn test_path_alias() {
        let root = Path::Root;
        let path = Path::Alias { parent: &root };
        assert_eq!(format!("{}", path), "");
    }

    #[test]
    fn test_path_unknown() {
        let root = Path::Root;
        let path = Path::Unknown { parent: &root };
        assert_eq!(format!("{}", path), "?");
    }

    #[test]
    fn test_path_nested() {
        let root = Path::Root;
        let seq = Path::Seq {
            parent: &root,
            index: 0,
        };
        let map = Path::Map {
            parent: &seq,
            key: "key",
        };
        let alias = Path::Alias { parent: &map };
        let unknown = Path::Unknown { parent: &alias };
        assert_eq!(format!("{}", unknown), "\\[0\\].key..?");
    }
}
