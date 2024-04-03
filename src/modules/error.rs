// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

use crate::{
    libyaml::{emitter, error as libyaml},
    modules::path::Path,
};
use serde::{de, ser};
use std::{
    error::Error as StdError,
    fmt::{self, Debug, Display},
    io, result, string,
    sync::Arc,
};

/// An error that happened serializing or deserializing YAML data.
pub struct Error(Box<ErrorImpl>);

/// Alias for a `Result` with the error type `serde_yml::Error`.
pub type Result<T> = result::Result<T, Error>;

/// The internal representation of an error.
#[derive(Debug)]
pub enum ErrorImpl {
    /// A generic error message with an optional position.
    Message(String, Option<Pos>),
    /// An error originating from the `libyaml` library.
    Libyaml(libyaml::Error),
    /// An I/O error.
    Io(io::Error),
    /// An error encountered while converting a byte slice to a string using UTF-8 encoding.
    FromUtf8(string::FromUtf8Error),
    /// An error indicating that the end of the YAML stream was reached unexpectedly.
    EndOfStream,
    /// An error indicating that more than one YAML document was encountered.
    MoreThanOneDocument,
    /// An error indicating that the recursion limit was exceeded.
    RecursionLimitExceeded(libyaml::Mark),
    /// An error indicating that the repetition limit was exceeded.
    RepetitionLimitExceeded,
    /// An error indicating that byte-based YAML is unsupported.
    BytesUnsupported,
    /// An error indicating that an unknown anchor was encountered.
    UnknownAnchor(libyaml::Mark),
    /// An error indicating that serializing a nested enum is not supported.
    SerializeNestedEnum,
    /// An error indicating that a scalar value was encountered in a merge operation.
    ScalarInMerge,
    /// An error indicating that a tagged value was encountered in a merge operation.
    TaggedInMerge,
    /// An error indicating that a scalar value was encountered in a merge element.
    ScalarInMergeElement,
    /// An error indicating that a sequence was encountered in a merge element.
    SequenceInMergeElement,
    /// An error indicating that an empty tag was encountered.
    EmptyTag,
    /// An error indicating that parsing a number failed.
    FailedToParseNumber,
    /// A shared error implementation.
    Shared(Arc<ErrorImpl>),
}

/// Represents a position in the YAML input.
#[derive(Debug)]
pub struct Pos {
    /// The mark representing the position.
    mark: libyaml::Mark,
    /// The path to the position.
    path: String,
}

/// The input location that an error occurred.
#[derive(Debug)]
pub struct Location {
    /// The byte index of the error.
    index: usize,
    /// The line of the error.
    line: usize,
    /// The column of the error.
    column: usize,
}

impl Location {
    /// The byte index of the error.
    pub fn index(&self) -> usize {
        self.index
    }

    /// The line of the error.
    pub fn line(&self) -> usize {
        self.line
    }

    /// The column of the error.
    pub fn column(&self) -> usize {
        self.column
    }

    // This is to keep decoupled with the yaml crate.
    #[doc(hidden)]
    fn from_mark(mark: libyaml::Mark) -> Self {
        Location {
            index: mark.index() as usize,
            // `line` and `column` returned from libyaml are 0-indexed but all error messages add +1 to this value.
            line: mark.line() as usize + 1,
            column: mark.column() as usize + 1,
        }
    }
}

impl Error {
    /// Returns the Location from the error if one exists.
    ///
    /// Not all types of errors have a location so this can return `None`.
    pub fn location(&self) -> Option<Location> {
        self.0.location()
    }

    /// Creates a new `Error` from an `ErrorImpl`.
    pub fn shared(self) -> Arc<ErrorImpl> {
        if let ErrorImpl::Shared(err) = *self.0 {
            err
        } else {
            Arc::from(self.0)
        }
    }
}

/// Creates a new `Error` from an `ErrorImpl`.
pub fn new(inner: ErrorImpl) -> Error {
    Error(Box::new(inner))
}

/// Creates a new `Error` from a shared `ErrorImpl`.
pub fn shared(shared: Arc<ErrorImpl>) -> Error {
    Error(Box::new(ErrorImpl::Shared(shared)))
}

/// Fixes the mark and path in an error.
pub fn fix_mark(
    mut error: Error,
    mark: libyaml::Mark,
    path: Path<'_>,
) -> Error {
    if let ErrorImpl::Message(_, none @ None) = error.0.as_mut() {
        *none = Some(Pos {
            mark,
            path: path.to_string(),
        });
    }
    error
}

impl From<libyaml::Error> for Error {
    fn from(err: libyaml::Error) -> Self {
        Error(Box::new(ErrorImpl::Libyaml(err)))
    }
}

impl From<emitter::Error> for Error {
    fn from(err: emitter::Error) -> Self {
        match err {
            emitter::Error::Libyaml(err) => Self::from(err),
            emitter::Error::Io(err) => new(ErrorImpl::Io(err)),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.0.source()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.display(f)
    }
}

// Remove two layers of verbosity from the debug representation. Humans often
// end up seeing this representation because it is what unwrap() shows.
impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.debug(f)
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error(Box::new(ErrorImpl::Message(msg.to_string(), None)))
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error(Box::new(ErrorImpl::Message(msg.to_string(), None)))
    }
}

impl ErrorImpl {
    fn location(&self) -> Option<Location> {
        self.mark().map(Location::from_mark)
    }

    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ErrorImpl::Io(err) => err.source(),
            ErrorImpl::FromUtf8(err) => err.source(),
            ErrorImpl::Shared(err) => err.source(),
            _ => None,
        }
    }

    fn mark(&self) -> Option<libyaml::Mark> {
        match self {
            ErrorImpl::Message(_, Some(Pos { mark, path: _ }))
            | ErrorImpl::RecursionLimitExceeded(mark)
            | ErrorImpl::UnknownAnchor(mark) => Some(*mark),
            ErrorImpl::Libyaml(err) => Some(err.mark()),
            ErrorImpl::Shared(err) => err.mark(),
            _ => None,
        }
    }

    fn message_no_mark(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            ErrorImpl::Message(msg, None) => f.write_str(msg),
            ErrorImpl::Message(msg, Some(Pos { mark: _, path })) => {
                if path != "." {
                    write!(f, "{}: ", path)?;
                }
                f.write_str(msg)
            }
            ErrorImpl::Libyaml(_) => unreachable!(),
            ErrorImpl::Io(err) => Display::fmt(err, f),
            ErrorImpl::FromUtf8(err) => Display::fmt(err, f),
            ErrorImpl::EndOfStream => f.write_str("EOF while parsing a value"),
            ErrorImpl::MoreThanOneDocument => f.write_str(
                "deserializing from YAML containing more than one document is not supported",
            ),
            ErrorImpl::RecursionLimitExceeded(_mark) => f.write_str("recursion limit exceeded"),
            ErrorImpl::RepetitionLimitExceeded => f.write_str("repetition limit exceeded"),
            ErrorImpl::BytesUnsupported => {
                f.write_str("serialization and deserialization of bytes in YAML is not implemented")
            }
            ErrorImpl::UnknownAnchor(_mark) => f.write_str("unknown anchor"),
            ErrorImpl::SerializeNestedEnum => {
                f.write_str("serializing nested enums in YAML is not supported yet")
            }
            ErrorImpl::ScalarInMerge => {
                f.write_str("expected a mapping or list of mappings for merging, but found scalar")
            }
            ErrorImpl::TaggedInMerge => f.write_str("unexpected tagged value in merge"),
            ErrorImpl::ScalarInMergeElement => {
                f.write_str("expected a mapping for merging, but found scalar")
            }
            ErrorImpl::SequenceInMergeElement => {
                f.write_str("expected a mapping for merging, but found sequence")
            }
            ErrorImpl::EmptyTag => f.write_str("empty YAML tag is not allowed"),
            ErrorImpl::FailedToParseNumber => f.write_str("failed to parse YAML number"),
            ErrorImpl::Shared(_) => unreachable!(),
        }
    }

    fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorImpl::Libyaml(err) => Display::fmt(err, f),
            ErrorImpl::Shared(err) => err.display(f),
            _ => {
                self.message_no_mark(f)?;
                if let Some(mark) = self.mark() {
                    if mark.line() != 0 || mark.column() != 0 {
                        write!(f, " at {}", mark)?;
                    }
                }
                Ok(())
            }
        }
    }

    fn debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorImpl::Libyaml(err) => Debug::fmt(err, f),
            ErrorImpl::Shared(err) => err.debug(f),
            _ => {
                f.write_str("Error(")?;
                struct MessageNoMark<'a>(&'a ErrorImpl);
                impl Display for MessageNoMark<'_> {
                    fn fmt(
                        &self,
                        f: &mut fmt::Formatter<'_>,
                    ) -> fmt::Result {
                        self.0.message_no_mark(f)
                    }
                }
                let msg = MessageNoMark(self).to_string();
                Debug::fmt(&msg, f)?;
                if let Some(mark) = self.mark() {
                    write!(
                        f,
                        ", line: {}, column: {}",
                        mark.line() + 1,
                        mark.column() + 1,
                    )?;
                }
                f.write_str(")")
            }
        }
    }
}
