// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

use crate::{
    de::{Event, Progress},
    libyaml::{
        error::Mark,
        parser::{Event as YamlEvent, Parser},
    },
    modules::error::{self, Error, ErrorImpl, Result},
};
use std::{borrow::Cow, collections::BTreeMap, sync::Arc};

/// Represents a YAML loader.
pub(crate) struct Loader<'input> {
    parser: Option<Parser<'input>>,
    document_count: usize,
}

/// Represents a YAML document.
pub(crate) struct Document<'input> {
    /// The parsed events of the document.
    pub events: Vec<(Event<'input>, Mark)>,
    /// Any error encountered during parsing.
    pub error: Option<Arc<ErrorImpl>>,
    /// Map from alias id to index in events.
    pub aliases: BTreeMap<usize, usize>,
}

impl<'input> Loader<'input> {
    /// Constructs a new `Loader` instance from the given progress.
    ///
    /// # Arguments
    ///
    /// * `progress` - The progress representing the YAML input.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue reading the input.
    pub fn new(progress: Progress<'input>) -> Result<Self> {
        let input = match progress {
            Progress::Str(s) => Cow::Borrowed(s.as_bytes()),
            Progress::Slice(bytes) => Cow::Borrowed(bytes),
            Progress::Read(mut rdr) => {
                let mut buffer = Vec::new();
                if let Err(io_error) = rdr.read_to_end(&mut buffer) {
                    return Err(error::new(ErrorImpl::Io(io_error)));
                }
                Cow::Owned(buffer)
            }
            Progress::Iterable(_) | Progress::Document(_) => {
                unreachable!()
            }
            Progress::Fail(err) => return Err(error::shared(err)),
        };

        Ok(Loader {
            parser: Some(Parser::new(input)),
            document_count: 0,
        })
    }

    /// Advances the loader to the next document and returns it.
    ///
    /// # Returns
    ///
    /// Returns `Some(Document)` if a document is successfully parsed, or `None` if there are no more documents.
    pub fn next_document(&mut self) -> Option<Document<'input>> {
        let parser = match &mut self.parser {
            Some(parser) => parser,
            None => return None,
        };

        let first = self.document_count == 0;
        self.document_count += 1;

        let mut anchors = BTreeMap::new();
        let mut document = Document {
            events: Vec::new(),
            error: None,
            aliases: BTreeMap::new(),
        };

        loop {
            let (event, mark) = match parser.next() {
                Ok((event, mark)) => (event, mark),
                Err(err) => {
                    document.error = Some(Error::from(err).shared());
                    return Some(document);
                }
            };
            let event = match event {
                YamlEvent::StreamStart => continue,
                YamlEvent::StreamEnd => {
                    self.parser = None;
                    return if first {
                        if document.events.is_empty() {
                            document.events.push((Event::Void, mark));
                        }
                        Some(document)
                    } else {
                        None
                    };
                }
                YamlEvent::DocumentStart => continue,
                YamlEvent::DocumentEnd => return Some(document),
                YamlEvent::Alias(alias) => match anchors.get(&alias) {
                    Some(id) => Event::Alias(*id),
                    None => {
                        document.error = Some(
                            error::new(ErrorImpl::UnknownAnchor(mark))
                                .shared(),
                        );
                        return Some(document);
                    }
                },
                YamlEvent::Scalar(mut scalar) => {
                    if let Some(anchor) = scalar.anchor.take() {
                        let id = anchors.len();
                        anchors.insert(anchor, id);
                        document
                            .aliases
                            .insert(id, document.events.len());
                    }
                    Event::Scalar(scalar)
                }
                YamlEvent::SequenceStart(mut sequence_start) => {
                    if let Some(anchor) = sequence_start.anchor.take() {
                        let id = anchors.len();
                        anchors.insert(anchor, id);
                        document
                            .aliases
                            .insert(id, document.events.len());
                    }
                    Event::SequenceStart(sequence_start)
                }
                YamlEvent::SequenceEnd => Event::SequenceEnd,
                YamlEvent::MappingStart(mut mapping_start) => {
                    if let Some(anchor) = mapping_start.anchor.take() {
                        let id = anchors.len();
                        anchors.insert(anchor, id);
                        document
                            .aliases
                            .insert(id, document.events.len());
                    }
                    Event::MappingStart(mapping_start)
                }
                YamlEvent::MappingEnd => Event::MappingEnd,
            };
            document.events.push((event, mark));
        }
    }
}
