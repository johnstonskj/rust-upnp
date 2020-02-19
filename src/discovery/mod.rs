/*!
This module implements the UPnP discovery protocol providing availability notifications and
search capabilities.

# Specification

This section explains the UPnP discovery protocol known as _Simple Service Discovery Protocol_
(SSDP) in detail, enumerating how devices advertise and revoke their advertisements as well as
how control points search and devices respond.

Discovery is the first step in UPnP networking. When a device is added to the network, the UPnP
discovery protocol allows that device to advertise its services to control points on the network.
Similarly, when a control point is added to the network, the UPnP discovery protocol allows that
control point to search for devices of interest on the network. The fundamental exchange in both
cases is a discovery message containing a few, essential specifics about the device or one of its
services, e.g., its type, universally unique identifier, and a pointer to more detailed information.

When a new device is added to the network, it multicasts a number of discovery messages advertising
itself, its embedded devices, and its services. Any interested control point can listen to the
standard multicast address for notifications that new capabilities are available.

Similarly, when a new control point is added to the network, it multicasts a discovery message
searching for interesting devices, services, or both. All devices must listen to the standard
multicast address for these messages and must respond if any of their embedded devices or services
match the search criteria in the discovery message.

To reiterate, a control point may learn of a device of interest because that device sent discovery
messages advertising itself or because the device responded to a discovery message searching for
devices. In either case, if a control point is interested in a device and wants to learn more about
it, the control point uses the information in the discovery message to send a description query
message. The section on Description explains description messages in detail.
11

When a device is removed from the network, it should, if possible, multicast a number of discovery
messages revoking its earlier announcements, effectively declaring that its embedded devices and
services will no longer be available. When the IP address of a device is changed, it should revoke
any earlier announcements and advertise using the new IP address.

For devices and control points that have multiple network interfaces, UPnP advertisements and
searches should be sent on all network interfaces enabled for UPnP networking. Each advertisement
or search must specify an address in the `LOCATION` header that is reachable on that interface

To limit network congestion, the time-to-live (TTL) of each IP packet for each multicast message
should default to 4 and should be configurable. When the TTL is greater than 1, it is possible for
multicast messages to traverse multiple routers; therefore control points and devices using
non-AutoIP addresses must send an IGMP Join message so that routers will forward multicast messages
to them (this is not necessary when using an Auto-IP address, since packets with Auto-IP addresses
will not be forwarded by routers).

Discovery plays an important role in the interoperability of devices and control points using
different versions of UPnP networking. The UPnP Device Architecture (defined herein) is versioned
with both a major and a minor version, usually written as major.minor, where both major and minor
are integers (for example, version 2.10 is newer than version 2.2). Advances in minor versions must
be a compatible superset of earlier minor versions of the same major version. Advances in major
version are not required to be supersets of earlier versions and are not guaranteed to be backward
compatible. Version information is communicated in discovery and description messages. In the
former, each discovery message includes the version of UPnP networking that the device supports
(in the `SERVER` header); the version of device and service types supported is also included in
relevant discovery messages. As a backup, the latter also includes the same information. This
section explains the format of version information in discovery messages and specific requirements
on discovery messages to maintain compatibility with advances in minor versions.
*/

use crate::{SpecVersion, UPNP_STRING};
use std::fmt::{Display, Error, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This represents a specific control point, this is optional for v1.0 and v1.1 messages
/// but the `friendly_name` field is required by the 2.0 specification.
///
#[derive(Clone, Debug)]
pub struct ControlPoint {
    /// Specifies the friendly name of the control point. The friendly name is vendor specific.
    pub friendly_name: String,
    /// UUID of the control point. When the control point is implemented in a UPnP device it
    /// is recommended to use the UDN of the co-located UPnP device.
    pub uuid: Option<String>,
    /// A control point can request that a device replies to a TCP port on the control point.
    /// When this header is present it identifies the TCP port on which the device can reply to
    /// the search.
    pub port: Option<u16>,
}

///
/// A product name and version, used in constructing `SERVER` and `CACHE-CONTROL` headers. These
/// arespecified by UPnP vendor. String.
///
/// Field value MUST begin with the following "product tokens" (defined by HTTP/1.1).
///
#[derive(Clone, Debug)]
pub struct ProductVersion {
    /// The name part of the product token
    pub name: String,
    /// The version part of the product token
    pub version: String,
}

///
/// The set of three products, and associated version identifiers, present in both `SERVER` and
/// `CACHE-CONTROL` headers.
///
#[derive(Clone, Debug)]
pub struct ProductVersions {
    pub operating_system: ProductVersion,
    pub upnp: ProductVersion,
    pub product: ProductVersion,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for ProductVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}/{}", self.name, self.version)
    }
}

impl ProductVersion {
    pub const fn new() -> Self {
        ProductVersion {
            name: String::new(),
            version: String::new(),
        }
    }

    pub fn for_upnp(version: &SpecVersion) -> Self {
        ProductVersion {
            name: UPNP_STRING.to_string(),
            version: version.to_string(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ProductVersions {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{} {} {}",
            self.operating_system, self.upnp, self.product
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod search;

pub mod notify;

mod protocol;
