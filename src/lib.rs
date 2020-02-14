/*!

_UPnP Device Architecture_ (UDA).

# Supported Components

* _Simple Service Discovery Protocol_ (SSDP)
* _Service Control Protocol Description_ (SCPD)
* _General Event Notification Architecture_ (GENA)
* _Simple Object Access Protocol_ (SOAP)

# More Information

* [UPnP Device Architecture 1.0 ](http://www.upnp.org/specs/arch/UPnP-arch-DeviceArchitecture-v1.0.pdf)
*/

#![warn(
    missing_debug_implementations,
    //missing_docs,
    unused_extern_crates,
    rust_2018_idioms
)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate tracing;

use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use std::str::Utf8Error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum SpecVersion {
    V10,
    V11,
    V20,
}

#[derive(Clone, Debug)]
pub enum MessageErrorKind {
    InvalidResponseStatus,
    InvalidEncoding,
    VersionMismatch,
    InvalidHeaderFormat,
    MissingRequiredField,
    FieldTypeMismatch,
    InvalidFieldValue,
}

#[derive(Clone, Debug)]
pub enum Error {
    NetworkTransport(IOErrorKind),
    MessageFormat(MessageErrorKind),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<IOError> for Error {
    fn from(e: IOError) -> Self {
        Error::NetworkTransport(e.kind())
    }
}

impl From<Utf8Error> for Error {
    fn from(_: Utf8Error) -> Self {
        Error::MessageFormat(MessageErrorKind::InvalidEncoding)
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod gena;

mod httpu;

pub mod ssdp;

mod scpd;

mod soap;

mod utils;
