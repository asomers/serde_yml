// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

#[cfg(test)]
mod tests {
    use serde_yml::{
        de::{Event, Progress},
        modules::error::ErrorImpl,
        loader::Loader,
    };
    use std::str;

    #[test]
    fn test_loader_new() {
        let input = "key: value";
        let progress = Progress::Str(input);
        let loader = Loader::new(progress).unwrap();
        assert!(loader.parser.is_some());
        assert_eq!(loader.document_count, 0);
    }

    #[test]
    fn test_loader_multiple_documents() {
        let input = "---\nkey1: value1\n...\n---\nkey2: value2\n...";
        let progress = Progress::Str(input);
        let mut loader = Loader::new(progress).unwrap();

        let document1 = loader.next_document().unwrap();
        assert_eq!(document1.events.len(), 4);
        assert!(document1.error.is_none());
        assert_eq!(document1.aliases.len(), 0);

        let document2 = loader.next_document().unwrap();
        assert_eq!(document2.events.len(), 4);
        assert!(document2.error.is_none());
        assert_eq!(document2.aliases.len(), 0);

        assert!(loader.next_document().is_none());
    }

    #[test]
    fn test_loader_unknown_anchor() {
        let input = "*unknown";
        let progress = Progress::Str(input);
        let mut loader = Loader::new(progress).unwrap();

        let document = loader.next_document().unwrap();
        assert_eq!(document.events.len(), 0);
        assert!(document.error.is_some());
        assert_eq!(document.aliases.len(), 0);

        let error = document.error.unwrap();
        assert!(matches!(*error, ErrorImpl::UnknownAnchor(_)));
    }

    #[test]
    fn test_loader_anchor_and_alias() {
        let input = "---\nkey: &anchor value\nalias: *anchor\n...";
        let progress = Progress::Str(input);
        let mut loader = Loader::new(progress).unwrap();

        let document = loader.next_document().unwrap();
        assert_eq!(document.events.len(), 6);
        assert!(document.error.is_none());
        assert_eq!(document.aliases.len(), 1);

        let (event, _) = &document.events[1];
        if let Event::Scalar(scalar) = event {
            assert_eq!(str::from_utf8(&scalar.value).unwrap(), "key");
            assert_eq!(scalar.anchor, None);
        } else {
            panic!("Expected Event::Scalar");
        }

        let (event, _) = &document.events[3];
        if let Event::Scalar(scalar) = event {
            assert_eq!(str::from_utf8(&scalar.value).unwrap(), "alias");
            assert_eq!(scalar.anchor, None);
        } else {
            panic!("Expected Event::Scalar");
        }

        let (event, _) = &document.events[4];
        assert!(matches!(event, Event::Alias(0)));
    }
}
