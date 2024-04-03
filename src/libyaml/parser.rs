// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

use crate::libyaml::{
    cstr::{self, CStr},
    error::{Error, Mark, Result},
    tag::Tag,
    util::Owned,
};
use std::{
    borrow::Cow,
    fmt::{self, Debug},
    mem::MaybeUninit,
    ptr::{addr_of_mut, NonNull},
    slice,
};
#[allow(clippy::unsafe_removed_from_name)]
use unsafe_libyaml as sys;

/// Represents a YAML parser.
///
/// The `Parser` struct is responsible for parsing YAML input and generating a sequence
/// of YAML events. It wraps the underlying `libyaml` parser and provides a safe and
/// convenient interface for parsing YAML documents.
///
/// The `'input` lifetime parameter indicates the lifetime of the input data being parsed.
/// It ensures that the `Parser` does not outlive the input data.
pub struct Parser<'input> {
    /// The pinned parser state.
    ///
    /// The `Owned<ParserPinned<'input>>` type represents an owned instance of the
    /// `ParserPinned` struct. The `Owned` type is used to provide pinning and
    /// allows the `Parser` to be safely moved around.
    ///
    /// The `ParserPinned` struct contains the underlying `libyaml` parser state
    /// and the input data being parsed.
    ///
    /// Pinning is used to ensure that the `Parser` remains at a fixed memory
    /// location, which is required for safe interaction with the `libyaml` library.
    pin: Owned<ParserPinned<'input>>,
}

struct ParserPinned<'input> {
    /// The underlying `yaml_parser_t` struct from the `libyaml` library.
    ///
    /// This field represents the low-level YAML parser state used by the `libyaml`
    /// library to parse YAML documents.
    sys: sys::yaml_parser_t,

    /// The input data being parsed.
    ///
    /// The `Cow<'input, [u8]>` type represents borrowed or owned input data.
    /// It allows the `Parser` to efficiently handle both borrowed slices and
    /// owned vectors of bytes.
    ///
    /// The `'input` lifetime parameter indicates the lifetime of the borrowed
    /// input data, if any.
    input: Cow<'input, [u8]>,
}

/// Represents a YAML event encountered during parsing.
#[derive(Debug)]
pub enum Event<'input> {
    /// Indicates the start of a YAML stream.
    StreamStart,

    /// Indicates the end of a YAML stream.
    StreamEnd,

    /// Indicates the start of a YAML document.
    DocumentStart,

    /// Indicates the end of a YAML document.
    DocumentEnd,

    /// Represents an alias event, referring to a previously defined anchor.
    ///
    /// The `Anchor` type represents the identifier of the alias.
    Alias(Anchor),

    /// Represents a scalar event, containing a scalar value.
    ///
    /// The `Scalar<'input>` type represents the scalar value and its associated
    /// properties, such as the anchor, tag, and style.
    ///
    /// The `'input` lifetime parameter indicates the lifetime of the input data
    /// associated with the scalar value.
    Scalar(Scalar<'input>),

    /// Represents the start of a sequence event.
    ///
    /// The `SequenceStart` type contains additional properties associated with
    /// the sequence, such as the anchor and tag.
    SequenceStart(SequenceStart),

    /// Indicates the end of a sequence event.
    SequenceEnd,

    /// Represents the start of a mapping event.
    ///
    /// The `MappingStart` type contains additional properties associated with
    /// the mapping, such as the anchor and tag.
    MappingStart(MappingStart),

    /// Indicates the end of a mapping event.
    MappingEnd,
}

/// Represents a scalar value in a YAML document.
pub struct Scalar<'input> {
    /// The anchor associated with the scalar value, if any.
    /// An anchor is a named reference to the scalar value that can be referred to later in the document.
    pub anchor: Option<Anchor>,

    /// The tag associated with the scalar value, if any.
    /// A tag specifies the data type or semantic meaning of the scalar value.
    pub tag: Option<Tag>,

    /// The actual value of the scalar, stored as a boxed slice of bytes.
    /// The value is stored as a byte slice to support arbitrary data types.
    pub value: Box<[u8]>,

    /// The style of the scalar value, indicating how it is represented in the YAML document.
    /// The `ScalarStyle` enum represents the different styles, such as plain, single-quoted, double-quoted, etc.
    pub style: ScalarStyle,

    /// The original representation of the scalar value in the YAML document, if available.
    /// This field is an optional reference to the original byte slice from the input.
    /// It can be used to preserve the exact formatting of the scalar value.
    pub repr: Option<&'input [u8]>,
}

/// Represents the start of a sequence in a YAML document.
#[derive(Debug)]
pub struct SequenceStart {
    /// The anchor associated with the sequence, if any.
    /// An anchor is a named reference to the sequence that can be referred to later in the document.
    pub anchor: Option<Anchor>,

    /// The tag associated with the sequence, if any.
    /// A tag specifies the data type or semantic meaning of the sequence.
    pub tag: Option<Tag>,
}

/// Represents the start of a mapping in a YAML document.
#[derive(Debug)]
pub struct MappingStart {
    /// The anchor associated with the mapping, if any.
    /// An anchor is a named reference to the mapping that can be referred to later in the document.
    pub anchor: Option<Anchor>,

    /// The tag associated with the mapping, if any.
    /// A tag specifies the data type or semantic meaning of the mapping.
    pub tag: Option<Tag>,
}

/// Represents an anchor in a YAML document.
/// An anchor is a named reference to a value that can be referred to later in the document.
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct Anchor(Box<[u8]>);

/// Represents the style of a scalar value in a YAML document.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ScalarStyle {
    /// Represents a plain scalar style, where the value is not enclosed in quotes.
    Plain,

    /// Represents a single-quoted scalar style, where the value is enclosed in single quotes.
    SingleQuoted,

    /// Represents a double-quoted scalar style, where the value is enclosed in double quotes.
    DoubleQuoted,

    /// Represents a literal scalar style, where the value is presented as a literal block.
    Literal,

    /// Represents a folded scalar style, where the value is presented as a folded block.
    Folded,
}

impl<'input> Parser<'input> {
    /// Creates a new `Parser` instance with the given input data.
    ///
    /// The `input` parameter is of type `Cow<'input, [u8]>`, which allows the parser
    /// to accept both borrowed slices and owned vectors of bytes as input.
    ///
    /// # Panics
    ///
    /// This function panics if there is an error initializing the underlying `libyaml` parser.
    pub fn new(input: Cow<'input, [u8]>) -> Parser<'input> {
        let owned = Owned::<ParserPinned<'input>>::new_uninit();
        let pin = unsafe {
            let parser = addr_of_mut!((*owned.ptr).sys);
            if sys::yaml_parser_initialize(parser).fail {
                panic!("malloc error: {}", Error::parse_error(parser));
            }
            sys::yaml_parser_set_encoding(parser, sys::YAML_UTF8_ENCODING);
            sys::yaml_parser_set_input_string(parser, input.as_ptr(), input.len() as u64);
            addr_of_mut!((*owned.ptr).input).write(input);
            Owned::assume_init(owned)
        };
        Parser { pin }
    }

    /// Parses the next YAML event from the input.
    ///
    /// Returns a `Result` containing the parsed `Event` and its corresponding `Mark` on success,
    /// or an `Error` if parsing fails.
    pub fn parse_next_event(&mut self) -> Result<(Event<'input>, Mark)> {
        let mut event = MaybeUninit::<sys::yaml_event_t>::uninit();
        unsafe {
            let parser = addr_of_mut!((*self.pin.ptr).sys);
            if (*parser).error != sys::YAML_NO_ERROR {
                return Err(Error::parse_error(parser));
            }
            let event = event.as_mut_ptr();
            if sys::yaml_parser_parse(parser, event).fail {
                return Err(Error::parse_error(parser));
            }
            let ret = convert_event(&*event, &(*self.pin.ptr).input);
            let mark = Mark {
                sys: (*event).start_mark,
            };
            sys::yaml_event_delete(event);
            Ok((ret, mark))
        }
    }
}

unsafe fn convert_event<'input>(
    sys: &sys::yaml_event_t,
    input: &Cow<'input, [u8]>,
) -> Event<'input> {
    match sys.type_ {
        sys::YAML_STREAM_START_EVENT => Event::StreamStart,
        sys::YAML_STREAM_END_EVENT => Event::StreamEnd,
        sys::YAML_DOCUMENT_START_EVENT => Event::DocumentStart,
        sys::YAML_DOCUMENT_END_EVENT => Event::DocumentEnd,
        sys::YAML_ALIAS_EVENT => Event::Alias(
            unsafe { optional_anchor(sys.data.alias.anchor) }.unwrap(),
        ),
        sys::YAML_SCALAR_EVENT => Event::Scalar(Scalar {
            anchor: unsafe { optional_anchor(sys.data.scalar.anchor) },
            tag: unsafe { optional_tag(sys.data.scalar.tag) },
            value: Box::from(unsafe {
                slice::from_raw_parts(
                    sys.data.scalar.value,
                    sys.data.scalar.length as usize,
                )
            }),
            #[allow(clippy::wildcard_in_or_patterns)]
            style: match unsafe { sys.data.scalar.style } {
                sys::YAML_PLAIN_SCALAR_STYLE => ScalarStyle::Plain,
                sys::YAML_SINGLE_QUOTED_SCALAR_STYLE => {
                    ScalarStyle::SingleQuoted
                }
                sys::YAML_DOUBLE_QUOTED_SCALAR_STYLE => {
                    ScalarStyle::DoubleQuoted
                }
                sys::YAML_LITERAL_SCALAR_STYLE => ScalarStyle::Literal,
                sys::YAML_FOLDED_SCALAR_STYLE => ScalarStyle::Folded,
                sys::YAML_ANY_SCALAR_STYLE | _ => unreachable!(),
            },
            repr: if let Cow::Borrowed(input) = input {
                Some(
                    &input[sys.start_mark.index as usize
                        ..sys.end_mark.index as usize],
                )
            } else {
                None
            },
        }),
        sys::YAML_SEQUENCE_START_EVENT => {
            Event::SequenceStart(SequenceStart {
                anchor: unsafe {
                    optional_anchor(sys.data.sequence_start.anchor)
                },
                tag: unsafe {
                    optional_tag(sys.data.sequence_start.tag)
                },
            })
        }
        sys::YAML_SEQUENCE_END_EVENT => Event::SequenceEnd,
        sys::YAML_MAPPING_START_EVENT => {
            Event::MappingStart(MappingStart {
                anchor: unsafe {
                    optional_anchor(sys.data.mapping_start.anchor)
                },
                tag: unsafe {
                    optional_tag(sys.data.mapping_start.tag)
                },
            })
        }
        #[allow(clippy::unnecessary_literal_unwrap)]
        sys::YAML_MAPPING_END_EVENT => Event::MappingEnd,
        sys::YAML_NO_EVENT => unreachable!(),
        _ => unreachable!(),
    }
}

unsafe fn optional_anchor(anchor: *const u8) -> Option<Anchor> {
    let ptr = NonNull::new(anchor as *mut i8)?;
    let cstr = { CStr::from_ptr(ptr) };
    Some(Anchor(Box::from(cstr.to_bytes())))
}

unsafe fn optional_tag(tag: *const u8) -> Option<Tag> {
    let ptr = NonNull::new(tag as *mut i8)?;
    let cstr = { CStr::from_ptr(ptr) };
    Some(Tag(Box::from(cstr.to_bytes())))
}

impl Debug for Scalar<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Scalar {
            anchor,
            tag,
            value,
            style,
            repr: _,
        } = self;

        struct LossySlice<'a>(&'a [u8]);

        impl Debug for LossySlice<'_> {
            fn fmt(
                &self,
                formatter: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                cstr::debug_lossy(self.0, formatter)
            }
        }

        formatter
            .debug_struct("Scalar")
            .field("anchor", anchor)
            .field("tag", tag)
            .field("value", &LossySlice(value))
            .field("style", style)
            .finish()
    }
}

impl Debug for Anchor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        cstr::debug_lossy(&self.0, formatter)
    }
}

impl Drop for ParserPinned<'_> {
    fn drop(&mut self) {
        unsafe { sys::yaml_parser_delete(&mut self.sys) }
    }
}
