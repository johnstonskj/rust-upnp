/**
Multicast channel and port reserved for SSDP by _Internet Assigned Numbers Authority_ (IANA). Must
be `239.255.255.250:1900`. If the port number (":1900") is omitted, the receiver should assume the
default SSDP port number of `1900`.
*/
pub const MULTICAST_ADDRESS: &str = "239.255.255.250:1900";

// ------------------------------------------------------------------------------------------------

/**
Required by HTTP Extension Framework. Unlike the NTS and ST headers, the value of the MAN header is
enclosed in double quotes; it defines the scope (namespace) of the extension. Must be
"ssdp:discover".
*/
pub const HTTP_EXTENSION: &str = "\"ssdp:discover\"";

// ------------------------------------------------------------------------------------------------

/**
From § 1.1 Discovery: Advertisement
*/
pub const METHOD_NOTIFY: &str = "NOTIFY";

/**
From § 1.2 Discovery: Search
*/
pub const METHOD_SEARCH: &str = "M-SEARCH";

// ------------------------------------------------------------------------------------------------

/**
The BOOTID.UPNP.ORG header field represents the boot instance of the device expressed according to
a monotonically increasing value. Its field value MUST be a non-negative 31-bit integer; ASCII
encoded, decimal, without leading zeros (leading zeroes, if present, MUST be ignored by the
recipient) that MUST be increased on each initial announce of the UPnP device or MUST be the same
as the field value of the NEXTBOOTID.UPNP.ORG header field in the last sent SSDP update message.
Its field value MUST remain the same on all periodically repeated announcements. A convenient
mechanism is to set this field value to the time that the device sends its initial announcement,
expressed as seconds elapsed since midnight January 1, 1970; for devices that have a notion of
time, this will not require any additional state to remember or be “flashed”. However, it is
perfectly acceptable to use a simple boot counter that is incremented on every initial announcement
as a field value of this header field. As such, control points MUST NOT view this header field as
a timestamp. The BOOTID.UPNP.ORG header field MUST be included in all announcements of a root
device, its embedded devices and its services. Unless the device explicitly updates its value by
sending an SSDP update message, as long as the device remains available in the network, the same
BOOTID.UPNP.ORG field value MUST be used in all announcements, search responses, update messages
and eventually bye-bye messages.
*/
pub const HEAD_BOOTID: &str = "BOOTID.UPNP.ORG";

/**
Field value MUST have the max-age directive ("max-age=") followed by an integer that specifies the
number of seconds the advertisement is valid. After this duration, control points SHOULD assume
the device (or service) is no longer available; as long as a control point has received at least
one advertisement that is still valid from a root device, any of its embedded devices or any of
its services, then the control point can assume that all are available. The number of seconds
SHOULD be greater than or equal to 1800 seconds (30 minutes), although exceptions are defined in
the text above. Specified by UPnP vendor. Other directives MUST NOT be sent and MUST be ignored
when received.
*/
pub const HEAD_CACHE_CONTROL: &str = "CACHE-CONTROL";

/**
The CONFIGID.UPNP.ORG field value MUST be a non-negative, 31-bit integer, ASCII encoded, decimal,
without leading zeros (leading zeroes, if present, MUST be ignored by the recipient) that MUST
represent the configuration number of a root device. UPnP 1.1 devices MAY freely assign configid
numbers from 0 to 16777215 (2^24-1). Higher numbers are reserved for future use, and can be
assigned by the Technical Committee. The configuration of a root device consists of the following
information: the DDD of the root device and all its embedded devices, and the SCPDs of all the
contained services. If any part of the configuration changes, the CONFIGID.UPNP.ORG field value
MUST be changed. The CONFIGID.UPNP.ORG header field MUST be included in all announcements of a
root device, its embedded devices and its services. The configuration number that is present in a
CONFIGID.UPNP.ORG field value MUST satisfy the following rule:

* if a device sends out two messages with a CONFIGID.UPNP.ORG header field with the same field
  value K, the configuration MUST be the same at the moments that these messages were sent.
*/
pub const HEAD_CONFIGID: &str = "CONFIGID.UPNP.ORG";

/**
Specifies the friendly name of the control point. The friendly name is vendor specific. When Device
Protection is implemented the cpfn.upnp.org shall be the same as the <Name> of Device Protection
unless the Device Protection <Alias> is defined, in which case it shall use the <Alias>.
*/
pub const HEAD_CP_FN: &str = "CPFN.UPNP.ORG";

/**
uuid of the control point. When the control point is implemented in a UPnP device it is recommended
to use the UDN of the co-located UPnP device. When implemented, all specified requirements for uuid
usage in devices also apply for control points.See section 1.1.4. Note that when Device Protection
is implemented the CPUUID.UPNP.ORG shall be the same as the uuid used in Device Protection.
*/
pub const HEAD_CP_UUID: &str = "CPUUID.UPNP.ORG";

/**
Field value contains date when response was generated. “rfc1123-date” as defined in RFC 2616.
*/
pub const HEAD_DATE: &str = "DATE";

/**
Required by HTTP Extension Framework. Confirms that the MAN header was understood. (Header only;
no value.)
*/
pub const HEAD_EXT: &str = "EXT";

/**
Field value contains multicast address and port reserved for SSDP by Internet Assigned Numbers
Authority (IANA). MUST be 239.255.255.250:1900. If the port number (“:1900”) is omitted, the
receiver MUST assume the default SSDP port number of 1900.
*/
pub const HEAD_HOST: &str = "HOST";

/**
Field value contains a URL to the UPnP description of the root device. Normally the host portion
contains a literal IP address rather than a domain name in unmanaged networks. Specified by UPnP
vendor. Single absolute URL (see RFC 3986).
*/
pub const HEAD_LOCATION: &str = "LOCATION";

/**
Required by HTTP Extension Framework. Unlike the NTS and ST headers, the value of the MAN header
is enclosed in double quotes; it defines the scope (namespace) of the extension. Must be
"ssdp:discover".
*/
pub const HEAD_MAN: &str = "MAN";

/**
Maximum wait time in seconds. Should be between 1 and 120 inclusive. Device responses
should be delayed a random duration between 0 and this many seconds to balance load for the control
point when it processes responses. This value may be increased if a large number of devices are
expected to respond. The MX value should not be increased to accommodate network characteristics
such as latency or propagation delay.
*/
pub const HEAD_MX: &str = "MX";

/**
Field value contains Notification Type. MUST be one of the following. (See Table 1-1, "Root device
discovery messages", Table 1-2, "Embedded device discovery messages", and Table 1-3, "Service
discovery messages") Single URI.
*/
pub const HEAD_NT: &str = "NT";

/**
Field value contains Notification Sub Type. Single URI.
*/
pub const HEAD_NTS: &str = "NTS";

/**
Field value contains the new BOOTID.UPNP.ORG field value that the device intends to use in the
subsequent device and service announcement messages. Its field value MUST be a non-negative 31-bit
integer; ASCII encoded, decimal, without leading zeros (leading zeroes, if present, MUST be ignored
by the recipient) and MUST be greater than the field value of the BOOTID.UPNP.ORG header field.
*/
pub const HEAD_NEXT_BOOTID: &str = "NEXTBOOTID.UPNP.ORG";

/**
If a device does not send the SEARCHPORT.UPNP.ORG header field, it MUST respond to unicast M-SEARCH
messages on port 1900. Only if port 1900 is unavailable MAY a device select a different port to
respond to unicast M-SEARCH messages. If a device sends the SEARCHPORT.UPNP.ORG header field, its
field value MUST be an ASCII encoded integer, decimal, without leading zeros (leading zeroes, if
present, MUST be ignored by the recipient), in the range 49152-65535 (RFC 4340). The device MUST
respond to unicast M-SEARCH messages that are sent to the advertised port
*/
pub const HEAD_SEARCH_PORT: &str = "SEARCHPORT.UPNP.ORG";

/**
Required when Device Protection is implemented.

The SECURELOCATION.UPNP.ORG header shall provide a base URL with “https:” for the scheme component
and indicate the correct “port” subcomponent in the “authority” component for a TLS connection.
Because the scheme and authority components are not included in relative URLs, these components are
obtained from the base URL provided by either LOCATION or SECURELOCATION.UPNP.ORG. See for more
information Ref DEVICEPROTECTION.
*/
pub const HEAD_SECURE_LOCATION: &str = "SECURELOCATION.UPNP.ORG";

/**
Specified by UPnP vendor. String. Field value MUST begin with the following "product tokens"
(defined by HTTP/1.1). The first product token identifes the operating system in the form OS
name/OS version, the second token represents the UPnP version and MUST be UPnP/1.1, and the third
token identifes the product using the form product name/product version. For example,
`SERVER: unix/5.1 UPnP/1.1 MyProduct/1.0`. Control points MUST be prepared to accept a higher minor
version number of the UPnP version than the control point itself implements. For example, control
points implementing UDA version 1.0 will be able to interoperate with devices implementing
UDA version 1.1.
*/
pub const HEAD_SERVER: &str = "SERVER";

/**
A control point can request that a device replies to a TCP port on the control point. When this
header is present it identifies the TCP port on which the device can reply to the search. If a
control point sends the TCPPORT.UPNP.ORG header field, its field value shall be an ASCII encoded
integer, decimal, without leading zeros (leading zeroes, if present, shall be ignored by the
recipient), in the range 49152-65535 (RFC 4340). The device shall respond to unicast M-SEARCH
messages similar to sending the response to the originating UDP port except that the notification
messages are sent to the advertised TCPPORT.UPNP.ORG port over TCP instead of UDP.
*/
pub const HEAD_TCP_PORT: &str = "TCPPORT.UPNP.ORG";

/**
Field value contains Search Target. Same as `HEAD_NT`.
*/
pub const HEAD_ST: &str = "ST";

/**
Specified by UPnP vendor. String. Field value MUST begin with the following “product tokens”
(defined by HTTP/1.1). The first product token identifes the operating system in the form OS
name/OS version, the second token represents the UPnP version and MUST be UPnP/1.1, and the third
token identifes the product using the form product name/product version. For example,
`USER-AGENT: unix/5.1 UPnP/1.1 MyProduct/1.0`. Control points MUST be prepared to accept a higher
minor version number of the UPnP version than the control point itself implements. For example,
control points implementing UDA version 1.0 will be able to interoperate with devices implementing
UDA version 1.1.
*/
pub const HEAD_USER_AGENT: &str = "USER-AGENT";

/**
Field value contains Unique Service Name. Identifies a unique instance of a device or service. MUST
be one of the following. (See Table 1-1, "Root device discovery messages", Table 1-2, "Embedded
device discovery messages", and Table 1-3, "Service discovery messages") The prefix (before the
double colon) MUST match the value of the UDN element in the device description. (Section 2,
"Description" explains the UDN element.) Single URI.
*/
pub const HEAD_USN: &str = "USN";

pub const NTS_ALIVE: &str = "ssdp:alive";

pub const NTS_BYE: &str = "ssdp:byebye";

pub const NTS_UPDATE: &str = "ssdp:update";
