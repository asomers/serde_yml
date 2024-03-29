// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

use crate::libyaml;
use crate::libyaml::util::Owned;
use std::ffi::c_void;
use std::io;
use std::mem::{self, MaybeUninit};
use std::ptr::{self, addr_of_mut};
use std::slice;
#[allow(clippy::unsafe_removed_from_name)]
use unsafe_libyaml as sys;

/// Errors that can occur during YAML emission.
#[derive(Debug)]
pub(crate) enum Error {
    /// Errors related to libyaml.
    Libyaml(libyaml::error::Error),
    /// I/O errors.
    Io(io::Error),
}

/// A YAML emitter.
pub(crate) struct Emitter<'a> {
    pin: Owned<EmitterPinned<'a>>,
}

struct EmitterPinned<'a> {
    sys: sys::yaml_emitter_t,
    write: Box<dyn io::Write + 'a>,
    write_error: Option<io::Error>,
}

/// YAML event types.
#[derive(Debug)]
pub(crate) enum Event<'a> {
    /// Start of a YAML stream.
    StreamStart,
    /// End of a YAML stream.
    StreamEnd,
    /// Start of a YAML document.
    DocumentStart,
    /// End of a YAML document.
    DocumentEnd,
    /// Scalar value.
    Scalar(Scalar<'a>),
    /// Start of a sequence.
    SequenceStart(Sequence),
    /// End of a sequence.
    SequenceEnd,
    /// Start of a mapping.
    MappingStart(Mapping),
    /// End of a mapping.
    MappingEnd,
}

/// Represents a scalar value in YAML.
#[derive(Debug)]
pub(crate) struct Scalar<'a> {
    /// Optional tag for the scalar.
    pub tag: Option<String>,
    /// Value of the scalar.
    pub value: &'a str,
    /// Style of the scalar.
    pub style: ScalarStyle,
}

/// Styles for YAML scalars.
#[derive(Debug)]
pub(crate) enum ScalarStyle {
    /// Any scalar style.
    Any,
    /// Plain scalar style.
    Plain,
    /// Single quoted scalar style.
    SingleQuoted,
    /// Literal scalar style.
    Literal,
}

/// Represents a YAML sequence.
#[derive(Debug)]
pub(crate) struct Sequence {
    /// Optional tag for the sequence.
    pub tag: Option<String>,
}

/// Represents a YAML mapping.
#[derive(Debug)]
pub(crate) struct Mapping {
    /// Optional tag for the mapping.
    pub tag: Option<String>,
}

impl<'a> Emitter<'a> {
    /// Creates a new YAML emitter.
    pub fn new(write: Box<dyn io::Write + 'a>) -> Emitter<'a> {
        let owned = Owned::<EmitterPinned<'a>>::new_uninit();
        let pin = unsafe {
            let emitter = addr_of_mut!((*owned.ptr).sys);
            if sys::yaml_emitter_initialize(emitter).fail {
                panic!("malloc error: {}", libyaml::Error::emit_error(emitter));
            }
            sys::yaml_emitter_set_unicode(emitter, true);
            sys::yaml_emitter_set_width(emitter, -1);
            addr_of_mut!((*owned.ptr).write).write(write);
            addr_of_mut!((*owned.ptr).write_error).write(None);
            sys::yaml_emitter_set_output(emitter, write_handler, owned.ptr.cast());
            Owned::assume_init(owned)
        };
        Emitter { pin }
    }

    /// Emits a YAML event.
    pub fn emit(&mut self, event: Event<'_>) -> Result<(), Error> {
        let mut sys_event = MaybeUninit::<sys::yaml_event_t>::uninit();
        let sys_event = sys_event.as_mut_ptr();
        unsafe {
            let emitter = addr_of_mut!((*self.pin.ptr).sys);
            let initialize_status = match event {
                Event::StreamStart => {
                    sys::yaml_stream_start_event_initialize(sys_event, sys::YAML_UTF8_ENCODING)
                }
                Event::StreamEnd => sys::yaml_stream_end_event_initialize(sys_event),
                Event::DocumentStart => {
                    let version_directive = ptr::null_mut();
                    let tag_directives_start = ptr::null_mut();
                    let tag_directives_end = ptr::null_mut();
                    let implicit = true;
                    sys::yaml_document_start_event_initialize(
                        sys_event,
                        version_directive,
                        tag_directives_start,
                        tag_directives_end,
                        implicit,
                    )
                }
                Event::DocumentEnd => {
                    let implicit = true;
                    sys::yaml_document_end_event_initialize(sys_event, implicit)
                }
                Event::Scalar(mut scalar) => {
                    let anchor = ptr::null();
                    let tag = scalar.tag.as_mut().map_or_else(ptr::null, |tag| {
                        tag.push('\0');
                        tag.as_ptr()
                    });
                    let value = scalar.value.as_ptr();
                    let length = scalar.value.len() as i32;
                    let plain_implicit = tag.is_null();
                    let quoted_implicit = tag.is_null();
                    let style = match scalar.style {
                        ScalarStyle::Any => sys::YAML_ANY_SCALAR_STYLE,
                        ScalarStyle::Plain => sys::YAML_PLAIN_SCALAR_STYLE,
                        ScalarStyle::SingleQuoted => sys::YAML_SINGLE_QUOTED_SCALAR_STYLE,
                        ScalarStyle::Literal => sys::YAML_LITERAL_SCALAR_STYLE,
                    };
                    sys::yaml_scalar_event_initialize(
                        sys_event,
                        anchor,
                        tag,
                        value,
                        length,
                        plain_implicit,
                        quoted_implicit,
                        style,
                    )
                }
                Event::SequenceStart(mut sequence) => {
                    let anchor = ptr::null();
                    let tag = sequence.tag.as_mut().map_or_else(ptr::null, |tag| {
                        tag.push('\0');
                        tag.as_ptr()
                    });
                    let implicit = tag.is_null();
                    let style = sys::YAML_ANY_SEQUENCE_STYLE;
                    sys::yaml_sequence_start_event_initialize(
                        sys_event, anchor, tag, implicit, style,
                    )
                }
                Event::SequenceEnd => sys::yaml_sequence_end_event_initialize(sys_event),
                Event::MappingStart(mut mapping) => {
                    let anchor = ptr::null();
                    let tag = mapping.tag.as_mut().map_or_else(ptr::null, |tag| {
                        tag.push('\0');
                        tag.as_ptr()
                    });
                    let implicit = tag.is_null();
                    let style = sys::YAML_ANY_MAPPING_STYLE;
                    sys::yaml_mapping_start_event_initialize(
                        sys_event, anchor, tag, implicit, style,
                    )
                }
                Event::MappingEnd => sys::yaml_mapping_end_event_initialize(sys_event),
            };
            if initialize_status.fail {
                return Err(Error::Libyaml(libyaml::Error::emit_error(emitter)));
            }
            if sys::yaml_emitter_emit(emitter, sys_event).fail {
                return Err(self.error());
            }
        }
        Ok(())
    }

    /// Flushes the YAML emitter.
    pub fn flush(&mut self) -> Result<(), Error> {
        unsafe {
            let emitter = addr_of_mut!((*self.pin.ptr).sys);
            if sys::yaml_emitter_flush(emitter).fail {
                return Err(self.error());
            }
        }
        Ok(())
    }

    /// Retrieves the inner writer from the YAML emitter.
    #[allow(unused_mut)]
    pub fn into_inner(mut self) -> Box<dyn io::Write + 'a> {
        let sink = Box::new(io::sink());
        unsafe { mem::replace(&mut (*self.pin.ptr).write, sink) }
    }

    /// Retrieves the error from the YAML emitter.
    fn error(&mut self) -> Error {
        let emitter = unsafe { &mut *self.pin.ptr };
        if let Some(write_error) = emitter.write_error.take() {
            Error::Io(write_error)
        } else {
            Error::Libyaml(unsafe { libyaml::Error::emit_error(&emitter.sys) })
        }
    }
}

/// Writes data to a buffer using a provided callback function.
unsafe fn write_handler(data: *mut c_void, buffer: *mut u8, size: u64) -> i32 {
    let data = data.cast::<EmitterPinned<'_>>();
    match io::Write::write_all(unsafe { &mut *(*data).write }, unsafe {
        slice::from_raw_parts(buffer, size as usize)
    }) {
        Ok(()) => 1,
        Err(err) => {
            unsafe {
                (*data).write_error = Some(err);
            }
            0
        }
    }
}

impl Drop for EmitterPinned<'_> {
    /// Drops the YAML emitter, deallocating resources.
    fn drop(&mut self) {
        unsafe { sys::yaml_emitter_delete(&mut self.sys) }
    }
}
