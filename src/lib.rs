/*!
* This crate implements the four major components of the _UPnP Device Architecture_ (UDA), namely
*
* 1. Discovery, the search and notification functions via the _Simple Service Discovery Protocol_
*    (SSDP).
* 2. Description, the device and service templates.
* 3. Control, TBD
* 4. Eventing, via the  _General Event Notification Architecture_ (GENA).
*
* The UDA covers the search for devices by a control point (client) as well as how devices respond
* with, and proactively advertise, capabilities. There are currently 3 versions of the UDA
* specification, v[1.0](http://www.upnp.org/specs/arch/UPnP-arch-DeviceArchitecture-v1.0.pdf),
* v[1.1](http://www.upnp.org/specs/arch/UPnP-arch-DeviceArchitecture-v1.1.pdf) (also standardized as
* ISO/IEC 29341-1-1), and v[2.0](http://www.upnp.org/specs/arch/UPnP-arch-DeviceArchitecture-v2.0.pdf).
*
* For more information see the specifications and other documents at [Open Connectivity
* Foundation](https://openconnectivity.org/developer/specifications/upnp-resources/upnp/).
*
* The main interface is the [`ssdp`](ssdp/index.html) module.
*
* # Example
*
* The following example issues a single v1.0 multicast search and collects and returns a set of
* device responses.
*
* ```rust,no_run
* use upnp_rs::SpecVersion;
* use upnp_rs::discovery::search::*;
*
* let mut options = Options::default_for(SpecVersion::V10);
* options.search_target = SearchTarget::RootDevice;
*
* match search_once(options) {
*     Ok(responses) => {
*         println!("search returned {} results.", responses.len());
*         for (index, response) in responses.iter().enumerate() {
*             println!("{}: {:#?}", index, response);
*         }
*     }
*     Err(error) => {
*         println!("search failed with error: {:#?}", error);
*     }
* }
* ```
*
* # Documentation
*
* Where possible any documentation for fields, functions, and values will be taken directly from the
* UDA specifications. In general the description will be taken from the version of the specication
* where the component in question was first introduced.
*
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
extern crate log;

use crate::common::xml;
use crate::description::xml::{X_ELEM_MAJOR, X_ELEM_MINOR, X_ELEM_SPEC_VERSION};
use quick_xml::Writer;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Write};
use std::str::{FromStr, Utf8Error};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This denotes the version of the _UPnP Device Architecture_ (UDA) specification to use
/// for a given interaction.
///
/// This allows the client to constrain the messaging to only the capabilities described by a
/// specific version.
///
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpecVersion {
    /// Denotes messages conforming to UPnP version
    /// [1.0](http://www.upnp.org/specs/arch/UPnP-arch-DeviceArchitecture-v1.0.pdf)
    V10,
    /// Denotes messages conforming to UPnP version
    /// [1.1](http://www.upnp.org/specs/arch/UPnP-arch-DeviceArchitecture-v1.1.pdf)
    V11,
    /// Denotes messages conforming to UPnP version
    /// [2.0](http://www.upnp.org/specs/arch/UPnP-arch-DeviceArchitecture-v2.0.pdf)
    V20,
}

///
/// This denotes the classes of errors that can arise while processing messages.
///
#[derive(Clone, Debug)]
pub enum MessageErrorKind {
    /// The response line of a message was invalid in some manner
    InvalidResponseStatus,
    /// The message could not be decoded from it's byte representation
    InvalidEncoding,
    /// The embedded version was incorrectly formatted
    InvalidVersion,
    /// The version in a message did not match the supported version
    VersionMismatch,
    /// A message header was incorrectly formatted
    InvalidHeaderFormat,
    /// A required field was either missing or empty
    MissingRequiredField,
    /// The field value did not match the expected type
    FieldTypeMismatch,
    /// The field value was not valid in the current context
    InvalidFieldValue,
}

///
/// This provides a common error type across the stack.
///
#[derive(Clone, Debug)]
pub enum Error {
    /// Some network error, more details provided by `std::io::Error`.
    NetworkTransport(IOErrorKind),
    Messaging,
    /// Message construction, parsing, or validation error, more details provided by
    /// `MessageErrorKind`.
    MessageFormat(MessageErrorKind),
    /// An operation you attempted is not supported.
    Unsupported,
}

///
/// The protocol short form identifier used in constructing numerous values.
///
pub const UPNP_STRING: &str = "UPnP";

///
/// The domain part of standard UPnP URI/URN identifiers.
///
pub const UPNP_DOMAIN: &str = "schemas-upnp-org";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SpecVersion {
    fn default() -> Self {
        SpecVersion::V10
    }
}

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

impl<T: Write> xml::Writable<T> for SpecVersion {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), quick_xml::Error> {
        let spec_version = xml::start_element(writer, X_ELEM_SPEC_VERSION)?;
        xml::text_element(
            writer,
            X_ELEM_MAJOR,
            match self {
                SpecVersion::V10 => "1",
                SpecVersion::V11 => "1",
                SpecVersion::V20 => "2",
            }
            .as_bytes(),
        )?;
        xml::text_element(
            writer,
            X_ELEM_MINOR,
            match self {
                SpecVersion::V10 => "0",
                SpecVersion::V11 => "1",
                SpecVersion::V20 => "0",
            }
            .as_bytes(),
        )?;
        spec_version.end(writer)
    }
}

// ------------------------------------------------------------------------------------------------

impl From<IOError> for Error {
    fn from(e: IOError) -> Self {
        error!("IO error: {:?}", e);
        Error::NetworkTransport(e.kind())
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        error!("UTF-8 encoding error: {:?}", e);
        Error::MessageFormat(MessageErrorKind::InvalidEncoding)
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod discovery;

pub mod description;

pub mod control;

pub mod eventing;

// ------------------------------------------------------------------------------------------------

pub(crate) mod common;
