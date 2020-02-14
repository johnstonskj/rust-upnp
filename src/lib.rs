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

use std::fmt::{Display, Error as FmtError, Formatter};
use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use std::str::{FromStr, Utf8Error};

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
    InvalidVersion,
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

impl Display for SpecVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(
            f,
            "{}",
            match self {
                SpecVersion::V10 => {
                    "1.0"
                }
                SpecVersion::V11 => {
                    "1.1"
                }
                SpecVersion::V20 => {
                    "2.0"
                }
            }
        )
    }
}

impl FromStr for SpecVersion {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1.0" => Ok(SpecVersion::V10),
            "1.1" => Ok(SpecVersion::V11),
            "2.0" => Ok(SpecVersion::V20),
            _ => Err(Error::MessageFormat(MessageErrorKind::InvalidVersion)),
        }
    }
}

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

pub mod ssdp;

mod gena;

mod httpu;

mod scpd;

mod soap;

mod utils;
