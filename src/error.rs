/*!
One-line description.
More detailed description, with
# Example
 */

use quick_xml::Error as XMLError;
use reqwest::Error as HTTPError;
use std::fmt::Display;
use std::io::Error as IOError;
use std::str::Utf8Error;
use thiserror::Error;

use crate::SpecVersion;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This provides a common error type across the stack.
///
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    NetworkTransport(#[from] IOError),

    #[error(transparent)]
    Messaging(#[from] HTTPError),

    #[error(transparent)]
    MessageFormat(#[from] MessageFormatError),

    #[error("An operation you attempted returned an error status `{status}` (Operation: `{0}`)")]
    OperationFailed { operation: String, status: String },

    #[error("The version supplied is valid, but not supported (Version: `{version}`)")]
    UnsupportedVersion { version: SpecVersion },

    #[error("An operation you attempted is not supported (Operation: `{operation}`)")]
    UnsupportedOperation { operation: String },
}

#[derive(Clone, Copy, Debug, Error)]
pub enum ValueSource {
    Socket,
    Header,
    Field,
}

#[derive(Debug, Error)]
pub enum MessageFormatError {
    #[error(transparent)]
    XmlFormat(#[from] XMLError),

    #[error(transparent)]
    SourceEncoding(#[from] Utf8Error),

    #[error("The version in a `{source}` did not match the supported version `{target}`")]
    VersionMismatch { source: ValueSource, target: String },

    #[error("A required {source} `{name}` was either missing or empty")]
    MissingRequiredValue { source: ValueSource, name: String },

    #[error("The {source} `{name}` value did not match the expected type (Expected: `{expected}`, Found: `{found}`)")]
    ValueTypeMismatch {
        source: ValueSource,
        name: String,
        expected: String,
        found: String,
    },

    #[error("The {source} `{name}` was incorrectly formatted (Value: `{value}`)")]
    InvalidValue {
        source: ValueSource,
        name: String,
        value: String,
    },

    #[error("The value provided is not valid for type `{for_type}` (Value: `{value}`)")]
    InvalidValueForType { for_type: String, value: String },
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn version_mismatch<S1, S2>(source: ValueSource, target: S2) -> MessageFormatError
where
    S1: Into<String>,
    S2: Into<String>,
{
    MessageFormatError::VersionMismatch {
        source,
        target: target.into(),
    }
}

pub fn missing_required_header<S>(name: S) -> MessageFormatError
where
    S: Into<String>,
{
    MessageFormatError::MissingRequiredValue {
        source: ValueSource::Header,
        name: name.into(),
    }
}

pub fn header_type_mismatch<S1, S2, S3>(name: S1, expected: S2, found: S3) -> MessageFormatError
where
    S1: Into<String>,
    S2: Into<String>,
    S3: Into<String>,
{
    MessageFormatError::ValueTypeMismatch {
        source: ValueSource::Header,
        name: name.into(),
        expected: expected.into(),
        found: found.into(),
    }
}

pub fn invalid_header_value<S1, S2>(name: S1, value: S2) -> MessageFormatError
where
    S1: Into<String>,
    S2: Into<String>,
{
    MessageFormatError::InvalidValue {
        source: ValueSource::Header,

        name: name.into(),
        value: value.into(),
    }
}

pub fn missing_required_field<S>(name: S) -> MessageFormatError
where
    S: Into<String>,
{
    MessageFormatError::MissingRequiredValue {
        source: ValueSource::Field,
        name: name.into(),
    }
}

pub fn field_type_mismatch<S1, S2, S3>(name: S1, expected: S2, found: S3) -> MessageFormatError
where
    S1: Into<String>,
    S2: Into<String>,
    S3: Into<String>,
{
    MessageFormatError::ValueTypeMismatch {
        source: ValueSource::Field,
        name: name.into(),
        expected: expected.into(),
        found: found.into(),
    }
}

pub fn invalid_field_value<S1, S2>(name: S1, value: S2) -> MessageFormatError
where
    S1: Into<String>,
    S2: Into<String>,
{
    MessageFormatError::InvalidValue {
        source: ValueSource::Field,
        name: name.into(),
        value: value.into(),
    }
}

pub fn invalid_socket_value<S1, S2>(name: S1, value: S2) -> MessageFormatError
where
    S1: Into<String>,
    S2: Into<String>,
{
    MessageFormatError::InvalidValue {
        source: ValueSource::Socket,
        name: name.into(),
        value: value.into(),
    }
}

pub fn unsupported_version(version: SpecVersion) -> Error {
    Error::UnsupportedVersion { version }
}

pub fn unsupported_operation<S1>(operation: S1) -> Error
where
    S1: Into<String>,
{
    Error::UnsupportedOperation {
        operation: operation.into(),
    }
}

pub fn invalid_value_for_type<S1, S2>(for_type: S1, value: S2) -> MessageFormatError
where
    S1: Into<String>,
    S2: Into<String>,
{
    MessageFormatError::InvalidValueForType {
        for_type: for_type.into(),
        value: value.into(),
    }
}

pub fn xml_error(e: XMLError) -> Error {
    Error::MessageFormat(MessageFormatError::XmlFormat(e))
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for ValueSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ValueSource::Socket => "socket",
                ValueSource::Header => "message header",
                ValueSource::Field => "message field",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl<T> From<MessageFormatError> for Result<T, MessageFormatError> {
    fn from(e: MessageFormatError) -> Self {
        Err(e)
    }
}

impl<T> From<MessageFormatError> for Result<T, Error> {
    fn from(e: MessageFormatError) -> Self {
        Err(Error::MessageFormat(e))
    }
}

impl<T> From<Error> for Result<T, Error> {
    fn from(e: Error) -> Self {
        Err(e)
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
