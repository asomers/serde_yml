#[cfg(test)]
mod tests {
    use serde_yml::{
        de::{Event, Progress},
        loader::Loader,
        modules::error::ErrorImpl,
    };
    use std::str;
    use std::sync::Arc;

    #[test]
    fn test_loader_new() {
        let input = "key: value";
        let progress = Progress::Str(input);
        let loader = Loader::new(progress).unwrap();
        assert!(loader.parser.is_some());
        assert_eq!(loader.parsed_document_count, 0);
    }

    #[test]
    fn test_loader_multiple_documents() {
        let input = "---\nkey1: value1\n...\n---\nkey2: value2\n...";
        let progress = Progress::Str(input);
        let mut loader = Loader::new(progress).unwrap();

        let document1 = loader.next_document().unwrap();
        assert_eq!(document1.events.len(), 4);
        assert!(document1.error.is_none());
        assert_eq!(document1.anchor_event_map.len(), 0);

        let document2 = loader.next_document().unwrap();
        assert_eq!(document2.events.len(), 4);
        assert!(document2.error.is_none());
        assert_eq!(document2.anchor_event_map.len(), 0);

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
        assert_eq!(document.anchor_event_map.len(), 0);

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
        assert_eq!(document.anchor_event_map.len(), 1);

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

    #[test]
    fn test_loader_empty_document() {
        let input = "---\n...";
        let progress = Progress::Str(input);
        let mut loader = Loader::new(progress).unwrap();

        let document = loader.next_document().unwrap();
        assert_eq!(document.events.len(), 1);
        assert!(document.error.is_none());
        assert_eq!(document.anchor_event_map.len(), 0);

        let (event, _) = &document.events[0];
        assert!(matches!(event, Event::Scalar(_)));
    }

    #[test]
    fn test_loader_sequence() {
        let input = "---\n- item1\n- item2\n...";
        let progress = Progress::Str(input);
        let mut loader = Loader::new(progress).unwrap();

        let document = loader.next_document().unwrap();
        assert_eq!(document.events.len(), 4);
        assert!(document.error.is_none());
        assert_eq!(document.anchor_event_map.len(), 0);

        let (event, _) = &document.events[0];
        assert!(matches!(event, Event::SequenceStart(_)));

        let (event, _) = &document.events[1];
        if let Event::Scalar(scalar) = event {
            assert_eq!(str::from_utf8(&scalar.value).unwrap(), "item1");
        } else {
            panic!("Expected Event::Scalar");
        }

        let (event, _) = &document.events[2];
        if let Event::Scalar(scalar) = event {
            assert_eq!(str::from_utf8(&scalar.value).unwrap(), "item2");
        } else {
            panic!("Expected Event::Scalar");
        }

        let (event, _) = &document.events[3];
        assert!(matches!(event, Event::SequenceEnd));
    }

    #[test]
    fn test_loader_mapping() {
        let input = "---\nkey1: value1\nkey2: value2\n...";
        let progress = Progress::Str(input);
        let mut loader = Loader::new(progress).unwrap();

        let document = loader.next_document().unwrap();
        assert_eq!(document.events.len(), 6);
        assert!(document.error.is_none());
        assert_eq!(document.anchor_event_map.len(), 0);

        let (event, _) = &document.events[0];
        assert!(matches!(event, Event::MappingStart(_)));

        let (event, _) = &document.events[1];
        if let Event::Scalar(scalar) = event {
            assert_eq!(str::from_utf8(&scalar.value).unwrap(), "key1");
        } else {
            panic!("Expected Event::Scalar");
        }

        let (event, _) = &document.events[2];
        if let Event::Scalar(scalar) = event {
            assert_eq!(
                str::from_utf8(&scalar.value).unwrap(),
                "value1"
            );
        } else {
            panic!("Expected Event::Scalar");
        }

        let (event, _) = &document.events[3];
        if let Event::Scalar(scalar) = event {
            assert_eq!(str::from_utf8(&scalar.value).unwrap(), "key2");
        } else {
            panic!("Expected Event::Scalar");
        }

        let (event, _) = &document.events[4];
        if let Event::Scalar(scalar) = event {
            assert_eq!(
                str::from_utf8(&scalar.value).unwrap(),
                "value2"
            );
        } else {
            panic!("Expected Event::Scalar");
        }

        let (event, _) = &document.events[5];
        assert!(matches!(event, Event::MappingEnd));
    }

    #[test]
    fn test_loader_escaped_characters() {
        let input = "---\nkey: \"value with \\\"quotes\\\"\"\n...";
        let progress = Progress::Str(input);
        let mut loader = Loader::new(progress).unwrap();

        let document = loader.next_document().unwrap();
        assert_eq!(document.events.len(), 4);
        assert!(document.error.is_none());
        assert_eq!(document.anchor_event_map.len(), 0);

        let (event, _) = &document.events[1];
        if let Event::Scalar(scalar) = event {
            assert_eq!(str::from_utf8(&scalar.value).unwrap(), "key");
        } else {
            panic!("Expected Event::Scalar");
        }

        let (event, _) = &document.events[2];
        if let Event::Scalar(scalar) = event {
            assert_eq!(
                str::from_utf8(&scalar.value).unwrap(),
                "value with \"quotes\""
            );
        } else {
            panic!("Expected Event::Scalar");
        }
    }

    #[test]
    fn test_loader_ignored_comments() {
        let input = "---\n# This is a comment\nkey: value # Inline comment\n...";
        let progress = Progress::Str(input);
        let mut loader = Loader::new(progress).unwrap();

        let document = loader.next_document().unwrap();
        assert_eq!(document.events.len(), 4); // Including comments
        assert!(document.error.is_none());
        assert_eq!(document.anchor_event_map.len(), 0);

        let (event, _) = &document.events[1];
        if let Event::Scalar(scalar) = event {
            assert_eq!(str::from_utf8(&scalar.value).unwrap(), "key");
        } else {
            panic!("Expected Event::Scalar");
        }

        let (event, _) = &document.events[2];
        if let Event::Scalar(scalar) = event {
            assert_eq!(str::from_utf8(&scalar.value).unwrap(), "value");
        } else {
            panic!("Expected Event::Scalar");
        }
    }
    #[test]
    fn test_loader_new_from_slice() {
        let input = "key: value".as_bytes();
        let progress = Progress::Slice(input);
        let loader = Loader::new(progress).unwrap();
        assert!(loader.parser.is_some());
        assert_eq!(loader.parsed_document_count, 0);
    }

    #[test]
    fn test_loader_new_from_reader() {
        use std::io::Cursor;

        let input = Cursor::new("key: value".as_bytes());
        let progress = Progress::Read(Box::new(input));
        let loader = Loader::new(progress).unwrap();
        assert!(loader.parser.is_some());
        assert_eq!(loader.parsed_document_count, 0);
    }

    #[test]
    fn test_loader_new_from_fail() {
        let error = ErrorImpl::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "test error",
        ));
        let progress = Progress::Fail(Arc::new(ErrorImpl::Shared(
            Arc::new(error),
        )));
        let loader_result = Loader::new(progress);
        assert!(loader_result.is_err());
    }

    #[test]
    fn test_loader_next_document_empty_input() {
        let input = "";
        let progress = Progress::Str(input);
        let mut loader = Loader::new(progress).unwrap();
        let document = loader.next_document().unwrap();
        assert_eq!(document.events.len(), 1);
        assert!(document.error.is_none());
        assert_eq!(document.anchor_event_map.len(), 0);
        assert!(loader.next_document().is_none());
    }
}
