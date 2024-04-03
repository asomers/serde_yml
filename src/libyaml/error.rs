// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

use crate::libyaml::cstr::CStr;
use std::{
    fmt::{self, Debug, Display},
    mem::MaybeUninit,
    ptr::NonNull,
};
#[allow(clippy::unsafe_removed_from_name)]
use unsafe_libyaml as sys;

/// A type alias for a `Result` with an `Error` as the error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents an error that occurred during YAML processing.
pub struct Error {
    /// The kind of error that occurred.
    ///
    /// This field uses the `yaml_error_type_t` type from the `unsafe_libyaml` crate,
    /// which represents different types of errors.
    kind: sys::yaml_error_type_t,

    /// A null-terminated string describing the problem that caused the error.
    ///
    /// The `CStr<'static>` type represents a borrowed C-style string with a static lifetime.
    problem: CStr<'static>,

    /// The offset of the problem that caused the error.
    problem_offset: u64,

    /// The mark indicating the position of the problem that caused the error.
    ///
    /// The `Mark` type represents a position in the YAML input.
    problem_mark: Mark,

    /// An optional null-terminated string providing additional context for the error.
    ///
    /// The `CStr<'static>` type represents a borrowed C-style string with a static lifetime.
    context: Option<CStr<'static>>,

    /// The mark indicating the position of the context related to the error.
    ///
    /// The `Mark` type represents a position in the YAML input.
    context_mark: Mark,
}

impl Error {
    /// Constructs an `Error` from a `yaml_parser_t` pointer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferences raw pointers and assumes
    /// the validity of the `yaml_parser_t` pointer.
    pub unsafe fn parse_error(parser: *const sys::yaml_parser_t) -> Self {
        Error {
            kind: unsafe { (*parser).error },
            problem: match NonNull::new(unsafe { (*parser).problem as *mut _ }) {
                Some(problem) => CStr::from_ptr(problem),
                None => CStr::from_bytes_with_nul(b"libyaml parser failed but there is no error\0"),
            },
            problem_offset: unsafe { (*parser).problem_offset },
            problem_mark: Mark {
                sys: unsafe { (*parser).problem_mark },
            },
            #[allow(clippy::manual_map)]
            context: match NonNull::new(unsafe { (*parser).context as *mut _ }) {
                Some(context) => Some(CStr::from_ptr(context)),
                None => None,
            },
            context_mark: Mark {
                sys: unsafe { (*parser).context_mark },
            },
        }
    }

    /// Constructs an `Error` from a `yaml_emitter_t` pointer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferences raw pointers and assumes
    /// the validity of the `yaml_emitter_t` pointer.
    pub unsafe fn emit_error(emitter: *const sys::yaml_emitter_t) -> Self {
        Error {
            kind: unsafe { (*emitter).error },
            problem: match NonNull::new(unsafe { (*emitter).problem as *mut _ }) {
                Some(problem) => CStr::from_ptr(problem),
                None => CStr::from_bytes_with_nul(b"libyaml emitter failed but there is no error\0"),
            },
            problem_offset: 0,
            problem_mark: Mark {
                sys: unsafe { MaybeUninit::<sys::yaml_mark_t>::zeroed().assume_init() },
            },
            context: None,
            context_mark: Mark {
                sys: unsafe { MaybeUninit::<sys::yaml_mark_t>::zeroed().assume_init() },
            },
        }
    }

    /// Returns the mark indicating the position of the problem that caused the error.
    pub fn mark(&self) -> Mark {
        self.problem_mark
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.problem)?;
        if self.problem_mark.sys.line != 0
            || self.problem_mark.sys.column != 0
        {
            write!(formatter, " at {}", self.problem_mark)?;
        } else if self.problem_offset != 0 {
            write!(formatter, " at position {}", self.problem_offset)?;
        }
        if let Some(context) = &self.context {
            write!(formatter, ", {}", context)?;
            if (self.context_mark.sys.line != 0
                || self.context_mark.sys.column != 0)
                && (self.context_mark.sys.line
                    != self.problem_mark.sys.line
                    || self.context_mark.sys.column
                        != self.problem_mark.sys.column)
            {
                write!(formatter, " at {}", self.context_mark)?;
            }
        }
        Ok(())
    }
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatter = formatter.debug_struct("Error");
        if let Some(kind) = match self.kind {
            sys::YAML_MEMORY_ERROR => Some("MEMORY"),
            sys::YAML_READER_ERROR => Some("READER"),
            sys::YAML_SCANNER_ERROR => Some("SCANNER"),
            sys::YAML_PARSER_ERROR => Some("PARSER"),
            sys::YAML_COMPOSER_ERROR => Some("COMPOSER"),
            sys::YAML_WRITER_ERROR => Some("WRITER"),
            sys::YAML_EMITTER_ERROR => Some("EMITTER"),
            _ => None,
        } {
            formatter.field("kind", &format_args!("{}", kind));
        }
        formatter.field("problem", &self.problem);
        if self.problem_mark.sys.line != 0
            || self.problem_mark.sys.column != 0
        {
            formatter.field("problem_mark", &self.problem_mark);
        } else if self.problem_offset != 0 {
            formatter.field("problem_offset", &self.problem_offset);
        }
        if let Some(context) = &self.context {
            formatter.field("context", context);
            if self.context_mark.sys.line != 0
                || self.context_mark.sys.column != 0
            {
                formatter.field("context_mark", &self.context_mark);
            }
        }
        formatter.finish()
    }
}

/// Represents a mark in a YAML document.
/// A mark indicates a specific position or location within the document.
#[derive(Copy, Clone)]
pub struct Mark {
    /// The underlying system representation of the mark.
    ///
    /// This field is marked as `pub(super)`, which means it is accessible within the current module
    /// and its parent module, but not from outside the crate.
    pub(super) sys: sys::yaml_mark_t,
}

impl Mark {
    /// Retrieves the index of the mark.
    ///
    /// The index represents the position of the mark within the YAML input.
    ///
    /// # Returns
    ///
    /// Returns the index of the mark as a `u64`.
    pub fn index(&self) -> u64 {
        self.sys.index
    }

    /// Retrieves the line number of the mark.
    ///
    /// The line number indicates the line in the YAML input where the mark is located.
    ///
    /// # Returns
    ///
    /// Returns the line number of the mark as a `u64`.
    pub fn line(&self) -> u64 {
        self.sys.line
    }

    /// Retrieves the column number of the mark.
    ///
    /// The column number indicates the column within the line where the mark is located.
    ///
    /// # Returns
    ///
    /// Returns the column number of the mark as a `u64`.
    pub fn column(&self) -> u64 {
        self.sys.column
    }
}

impl Display for Mark {
    /// Formats the mark for display purposes.
    ///
    /// If the line and column numbers are non-zero, the mark is formatted as "line X column Y".
    /// Otherwise, the mark is formatted as "position Z", where Z is the index.
    ///
    /// # Arguments
    ///
    /// * `formatter` - The formatter to write the display output to.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the formatting was successful, or an error otherwise.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.sys.line != 0 || self.sys.column != 0 {
            write!(
                formatter,
                "line {} column {}",
                self.sys.line + 1,
                self.sys.column + 1,
            )
        } else {
            write!(formatter, "position {}", self.sys.index)
        }
    }
}

impl Debug for Mark {
    /// Formats the mark for debugging purposes.
    ///
    /// The mark is formatted as a debug struct with either the line and column numbers
    /// or the index, depending on their values.
    ///
    /// # Arguments
    ///
    /// * `formatter` - The formatter to write the debug output to.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the formatting was successful, or an error otherwise.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatter = formatter.debug_struct("Mark");
        if self.sys.line != 0 || self.sys.column != 0 {
            formatter.field("line", &(self.sys.line + 1));
            formatter.field("column", &(self.sys.column + 1));
        } else {
            formatter.field("index", &self.sys.index);
        }
        formatter.finish()
    }
}
