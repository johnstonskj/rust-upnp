/**
Required. Multicast channel and port reserved for SSDP by Internet Assigned Numbers Authority (IANA). Must
be 239.255.255.250:1900. If the port number (“:1900”) is omitted, the receiver should assume the default SSDP po
number of 1900.
*/
pub const MULTICAST_ADDRESS: &str = "239.255.255.250:1900";

/**
From § 1.2.2 Discovery: Search: Request with M-SEARCH
*/
pub const MSG_SEARCH: &str = "M-SEARCH";

pub const HEAD_HOST: &str = "HOST";

pub const HEAD_LOCATION: &str = "LOCATION";

/**
Required by HTTP Extension Framework. Unlike the NTS and ST headers, the value of the MAN header is enclosed in
double quotes; it defines the scope (namespace) of the extension. Must be "ssdp:discover".
*/
pub const HEAD_MAN: &str = "MAN";

/**
Required. Maximum wait time in seconds. Should be between 1 and 120 inclusive. Device responses should be delayed a
random duration between 0 and this many seconds to balance load for the control point when it processes responses.
This value may be increased if a large number of devices are expected to respond. The MX value should not be
increased to accommodate network characteristics such as latency or propagation delay.
*/
pub const HEAD_MX: &str = "MX";

pub const HEAD_SERVER: &str = "SERVER";

pub const HEAD_USER_AGENT: &str = "USER-AGENT";

/**
Search Target
*/
pub const HEAD_ST: &str = "ST";

/**
Unique Service Name.
*/
pub const HEAD_USN: &str = "USN";

pub const HTTP_EXTENSION: &str = "\"ssdp:discover\"";
