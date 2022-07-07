/*!
This module provides three functions that provide 1) multicast search, 2) unicast search, and 3)
multicast search with caching. The caching version of search will merge the set of new responses
with any (non-expired) previously cached responses.

# Specification

TBD

*/
use crate::common::headers;
use crate::common::httpu::{
    multicast, Options as MulticastOptions, RequestBuilder, Response as MulticastResponse,
};
use crate::common::interface::IP;
use crate::common::uri::{URI, URL};
use crate::common::user_agent::user_agent_string;
use crate::discovery::{ControlPoint, ProductVersion, ProductVersions};
use crate::error::{
    invalid_field_value, invalid_header_value, invalid_value_for_type, missing_required_field,
    unsupported_operation, unsupported_version, Error, MessageFormatError,
};
use crate::syntax::{
    HTTP_EXTENSION, HTTP_HEADER_BOOTID, HTTP_HEADER_CACHE_CONTROL, HTTP_HEADER_CONFIGID,
    HTTP_HEADER_CP_FN, HTTP_HEADER_CP_UUID, HTTP_HEADER_DATE, HTTP_HEADER_EXT, HTTP_HEADER_HOST,
    HTTP_HEADER_LOCATION, HTTP_HEADER_MAN, HTTP_HEADER_MX, HTTP_HEADER_SEARCH_PORT,
    HTTP_HEADER_SERVER, HTTP_HEADER_ST, HTTP_HEADER_TCP_PORT, HTTP_HEADER_USER_AGENT,
    HTTP_HEADER_USN, HTTP_METHOD_SEARCH, MULTICAST_ADDRESS,
};
use crate::SpecVersion;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Error as FmtError, Formatter};
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::{Duration, SystemTime};
use tracing::{error, info, trace};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// `SearchTarget` corresponds to the set of values defined by the UDA `ST` header.
///
/// This type does not separate out the version of a device or service type, it does ensure
/// that the ':' separator character is present in the combined value.
///
#[derive(Clone, Debug)]
pub enum SearchTarget {
    /// Corresponds to the value `ssdp:all`
    All,
    /// Corresponds to the value `upnp:rootdevice`
    RootDevice,
    /// Corresponds to the value `uuid:{device-UUID}`
    Device(String),
    /// Corresponds to the value `urn:schemas-upnp-org:device:{deviceType:ver}`
    DeviceType(String),
    /// Corresponds to the value `urn:schemas-upnp-org:service:{serviceType:ver}`
    ServiceType(String),
    /// Corresponds to the value `urn:{domain-name}:device:{deviceType:ver}`
    DomainDeviceType(String, String),
    /// Corresponds to the value `urn:{domain-name}:service:{serviceType:ver}`
    DomainServiceType(String, String),
}

///
/// If present on a search function it will be called once for each received response
/// in addition to those responses being returned from the function.
///
/// The result is a boolean, if true the function will continue to process results,
/// if false no further responses are processed and the search will only return results
/// until this last one.
///
#[allow(dead_code)]
type CallbackFn = fn(&Response) -> bool;

///
/// This type encapsulates a set of mostly optional values to be used to construct messages to
/// send.
///
/// Defaults should be constructed with `Options::default_for`. Currently the only time a value
/// is required is when the version is set to 2.0, a value **is** required for the control point.
/// The `Options::for_control_point` will set the control point as well as the version number.
///
#[derive(Clone, Debug)]
pub struct Options {
    /// The specification that will be used to construct sent messages and to verify responses.
    /// Default: `SpecVersion:V10`.
    pub spec_version: SpecVersion,
    /// The scope of the search to perform. Default: `SearchTarget::RootDevices`.
    pub search_target: SearchTarget,
    /// A specific network interface to bind to; if specified the default address for the interface
    /// will be used, else the address `0.0.0.0:0` will be used. Default: `None`.
    pub network_interface: Option<String>,
    /// Denotes whether the implementation wants to only use IPv4, IPv6, or doesn't care.
    pub network_version: Option<IP>,
    /// The IP packet TTL value.
    pub packet_ttl: u32,
    /// The maximum wait time for devices to use in responding. This will also be used as the read
    /// timeout on the underlying socket. This value **must** be between `0` and `120`;
    /// default: `2`.
    pub max_wait_time: u8,
    /// If specified this is to be the `ProduceName/Version` component of the user agent string
    /// the client will generate as part of sent messages. If not specified a default value based
    /// on the name and version of this crate will be used. Default: `None`.
    pub product_and_version: Option<ProductVersion>,
    /// If specified this will be used to add certain control point values in the sent messages.
    /// This value is **only** used by the 2.0 specification where it is required, otherwise it
    /// will be ignores. Default: `None`.
    pub control_point: Option<ControlPoint>,
}

#[derive(Clone, Debug)]
struct CachedResponse {
    response: Response,
    #[allow(dead_code)]
    expiration: SystemTime,
}

///
/// A cache wrapping a set of responses.
///
#[derive(Clone, Debug)]
pub struct ResponseCache {
    #[allow(dead_code)]
    options: Options,
    #[allow(dead_code)]
    minimum_refresh: Duration,
    last_updated: SystemTime,
    responses: Vec<CachedResponse>,
}

///
/// A Single device response.
///
#[derive(Clone, Debug)]
pub struct Response {
    pub max_age: Duration,
    pub date: String,
    pub versions: ProductVersions,
    pub search_target: SearchTarget,
    pub service_name: URI,
    pub location: URL,
    pub boot_id: u64,
    pub config_id: Option<u64>,
    pub search_port: Option<u16>,
    pub other_headers: HashMap<String, String>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Perform a multicast search but store the results in a cache that allows a client to keep
/// the results around and use the `update` method to refresh the cache from the network.
///
/// The search function can be configured using the [`Options`](struct.Options.html) struct,
/// although the defaults are reasonable for most clients.
///
/// # Specification
///
/// TBD
///
/// # Parameters
///
/// * `options` - protocol options such as the specification version to use and any network
/// configuration values.
///
pub fn search(options: Options) -> Result<ResponseCache, Error> {
    info!("search - options: {:?}", options);
    options.validate()?;
    unsupported_operation("search").into()
}

///
/// Perform a multicast search but return the results immediately as a vector, not wrapped
/// in a cache.
///
/// The search function can be configured using the [`Options`](struct.Options.html) struct,
/// although the defaults are reasonable for most clients.
///
/// # Specification
///
/// TBD
///
/// # Parameters
///
/// * `options` - protocol options such as the specification version to use and any network
/// configuration values.
///
///
pub fn search_once(options: Options) -> Result<Vec<Response>, Error> {
    info!("search_once - options: {:?}", options);
    options.validate()?;
    let mut message_builder = RequestBuilder::new(HTTP_METHOD_SEARCH);
    // All headers from the original 1.0 specification.
    message_builder
        .add_header(HTTP_HEADER_HOST, MULTICAST_ADDRESS)
        .add_header(HTTP_HEADER_MAN, HTTP_EXTENSION)
        .add_header(HTTP_HEADER_MX, &format!("{}", options.max_wait_time))
        .add_header(HTTP_HEADER_ST, &options.search_target.to_string());
    // Headers added by 1.1 specification
    if options.spec_version >= SpecVersion::V11 {
        message_builder.add_header(
            HTTP_HEADER_USER_AGENT,
            &user_agent_string(options.spec_version, options.product_and_version.clone()),
        );
    }
    // Headers added by 2.0 specification
    if options.spec_version >= SpecVersion::V20 {
        match &options.control_point {
            Some(cp) => {
                message_builder.add_header(HTTP_HEADER_CP_FN, &cp.friendly_name);
                if let Some(uuid) = &cp.uuid {
                    message_builder.add_header(HTTP_HEADER_CP_UUID, uuid);
                }
                if let Some(port) = cp.port {
                    message_builder.add_header(HTTP_HEADER_TCP_PORT, &port.to_string());
                }
            }
            None => {
                error!("search_once - missing control point, required for UPnP/2.0");
                return missing_required_field("control_point").into();
            }
        }
    }
    trace!("search_once - {:?}", &message_builder);
    let raw_responses = multicast(
        &message_builder.into(),
        &MULTICAST_ADDRESS.parse().unwrap(),
        &options.into(),
    )?;

    let mut responses: Vec<Response> = Vec::new();
    for raw_response in raw_responses {
        responses.push(raw_response.try_into()?);
    }
    Ok(responses)
}

///
/// Perform a unicast search and return the results immediately as a vector, not wrapped
/// in a cache.
///
/// The search function can be configured using the [`Options`](struct.Options.html) struct,
/// although the defaults are reasonable for most clients.
///
/// # Specification
///
/// TBD
///
/// # Parameters
///
/// * `options` - protocol options such as the specification version to use and any network
/// configuration values.
/// * `device_address` - the address of the device to query.
///
///
pub fn search_once_to_device(
    options: Options,
    device_address: SocketAddr,
) -> Result<Vec<Response>, Error> {
    info!(
        "search_once_to_device - options: {:?}, device_address: {:?}",
        options, device_address
    );
    options.validate()?;
    if options.spec_version >= SpecVersion::V11 {
        let mut message_builder = RequestBuilder::new(HTTP_METHOD_SEARCH);
        message_builder
            .add_header(HTTP_HEADER_HOST, MULTICAST_ADDRESS)
            .add_header(HTTP_HEADER_MAN, HTTP_EXTENSION)
            .add_header(HTTP_HEADER_ST, &options.search_target.to_string())
            .add_header(
                HTTP_HEADER_USER_AGENT,
                &user_agent_string(options.spec_version, options.product_and_version.clone()),
            );

        let raw_responses = multicast(&message_builder.into(), &device_address, &options.into())?;

        let mut responses: Vec<Response> = Vec::new();
        for raw_response in raw_responses {
            responses.push(raw_response.try_into()?);
        }
        Ok(responses)
    } else {
        unsupported_version(options.spec_version).into()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SearchTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(
            f,
            "{}",
            match self {
                SearchTarget::All => "ssdp::all".to_string(),
                SearchTarget::RootDevice => "upnp:rootdevice".to_string(),
                SearchTarget::Device(device) => format!("uuid:{}", device),
                SearchTarget::DeviceType(device) =>
                    format!("urn:schemas-upnp-org:device:{}", device),
                SearchTarget::ServiceType(service) =>
                    format!("urn:schemas-upnp-org:service:{}", service),
                SearchTarget::DomainDeviceType(domain, device) =>
                    format!("urn:{}:device:{}", domain, device),
                SearchTarget::DomainServiceType(domain, service) =>
                    format!("urn:{}:service:{}", domain, service),
            }
        )
    }
}

impl FromStr for SearchTarget {
    type Err = MessageFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref DOMAIN_URN: Regex =
                Regex::new(r"^urn:([^:]+):(device|service):(.+)$").unwrap();
        }
        if s == "ssdp::all" {
            Ok(SearchTarget::All)
        } else if s == "upnp:rootdevice" {
            Ok(SearchTarget::RootDevice)
        } else if let Some(device) = s.strip_prefix("uuid:") {
            Ok(SearchTarget::Device(device.to_string()))
        } else if let Some(device_type) = s.strip_prefix("urn:schemas-upnp-org:device:") {
            Ok(SearchTarget::DeviceType(device_type.to_string()))
        } else if let Some(service_type) = s.strip_prefix("urn:schemas-upnp-org:service:") {
            Ok(SearchTarget::ServiceType(service_type.to_string()))
        } else if let Some(domain) = s.strip_prefix("urn:") {
            match DOMAIN_URN.captures(domain) {
                Some(captures) => {
                    if captures.get(2).unwrap().as_str() == "device" {
                        Ok(SearchTarget::DomainDeviceType(
                            captures.get(1).unwrap().as_str().to_string(),
                            captures.get(3).unwrap().as_str().to_string(),
                        ))
                    } else {
                        Ok(SearchTarget::DomainServiceType(
                            captures.get(1).unwrap().as_str().to_string(),
                            captures.get(3).unwrap().as_str().to_string(),
                        ))
                    }
                }
                None => {
                    error!("Could not parse URN '{}'", s);
                    invalid_value_for_type("URN", s).into()
                }
            }
        } else {
            error!("Could not parse '{}' as a search target", s);
            invalid_value_for_type("SearchTarget", s).into()
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Options {
    ///
    /// Construct an options object for the given specification version.
    ///
    pub fn default_for(spec_version: SpecVersion) -> Self {
        Options {
            spec_version,
            network_interface: None,
            network_version: None,
            search_target: SearchTarget::RootDevice,
            packet_ttl: if spec_version == SpecVersion::V10 {
                4
            } else {
                2
            },
            max_wait_time: 2,
            product_and_version: None,
            control_point: None,
        }
    }

    ///
    /// Construct an options object for the given control point.
    ///
    pub fn for_control_point(control_point: ControlPoint) -> Self {
        let mut new = Self::default_for(SpecVersion::V20);
        new.control_point = Some(control_point);
        new
    }

    ///
    /// Validate all options, ensuring values as well as version-specific rules.
    ///
    pub fn validate(&self) -> Result<(), Error> {
        lazy_static! {
            static ref UA_VERSION: Regex = Regex::new(r"^[\d\.]+$").unwrap();
        }
        if self.max_wait_time < 1 || self.max_wait_time > 120 {
            error!(
                "validate - max_wait_time must be between 1..120 ({})",
                self.max_wait_time
            );
            return invalid_field_value("max_wait_time", &self.max_wait_time.to_string()).into();
        }
        if self.spec_version >= SpecVersion::V11 {
            if let Some(user_agent) = &self.product_and_version {
                if user_agent.name.contains('/') || !UA_VERSION.is_match(&user_agent.version) {
                    error!(
                        "validate - user_agent needs to match 'ProductName/Version' ({:?})",
                        user_agent
                    );
                    return invalid_field_value("UserAgent", &user_agent.to_string()).into();
                }
            }
        }
        if self.spec_version >= SpecVersion::V20 {
            if self.control_point.is_none() {
                error!("validate - control_point required");
                return missing_required_field("ControlPoint").into();
            } else if let Some(control_point) = &self.control_point {
                if control_point.friendly_name.is_empty() {
                    error!("validate - control_point.friendly_name required");
                    return invalid_field_value("ControlPoint", &control_point.friendly_name)
                        .into();
                }
            }
        }
        Ok(())
    }
}

impl From<Options> for MulticastOptions {
    fn from(options: Options) -> Self {
        MulticastOptions {
            network_interface: options.network_interface,
            network_version: options.network_version,
            packet_ttl: options.packet_ttl,
            recv_timeout: options.max_wait_time as u64,
            ..Default::default()
        }
    }
}
// ------------------------------------------------------------------------------------------------

const REQUIRED_HEADERS_V10: [&str; 7] = [
    HTTP_HEADER_CACHE_CONTROL,
    HTTP_HEADER_DATE,
    HTTP_HEADER_EXT,
    HTTP_HEADER_LOCATION,
    HTTP_HEADER_SERVER,
    HTTP_HEADER_ST,
    HTTP_HEADER_USN,
];

impl TryFrom<MulticastResponse> for Response {
    type Error = Error;

    fn try_from(response: MulticastResponse) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref UA_ALL: Regex =
                Regex::new(r"^([^/]+)/([\d\.]+),?[ ]+([^/]+)/([\d\.]+),?[ ]+([^/]+)/([\d\.]+)$")
                    .unwrap();
        }
        headers::check_required(&response.headers, &REQUIRED_HEADERS_V10)?;
        headers::check_empty(
            response.headers.get(HTTP_HEADER_EXT).unwrap(),
            HTTP_HEADER_EXT,
        )?;

        let server = response.headers.get(HTTP_HEADER_SERVER).unwrap();
        let versions = match UA_ALL.captures(server) {
            Some(captures) => ProductVersions {
                product: ProductVersion {
                    name: captures.get(5).unwrap().as_str().to_string(),
                    version: captures.get(6).unwrap().as_str().to_string(),
                },
                upnp: ProductVersion {
                    name: captures.get(3).unwrap().as_str().to_string(),
                    version: captures.get(4).unwrap().as_str().to_string(),
                },
                platform: ProductVersion {
                    name: captures.get(1).unwrap().as_str().to_string(),
                    version: captures.get(2).unwrap().as_str().to_string(),
                },
            },
            None => {
                error!("invalid value for server header '{}", server);
                return invalid_field_value(HTTP_HEADER_SERVER, server).into();
            }
        };

        let max_age = headers::check_parsed_value::<u64>(
            &headers::check_regex(
                response.headers.get(HTTP_HEADER_CACHE_CONTROL).unwrap(),
                HTTP_HEADER_CACHE_CONTROL,
                &Regex::new(r"max-age[ ]*=[ ]*(\d+)").unwrap(),
            )?,
            HTTP_HEADER_CACHE_CONTROL,
        )?;

        let date = headers::check_not_empty(
            response.headers.get(HTTP_HEADER_DATE),
            "Thu, 01 Jan 1970 00:00:00 GMT",
        );

        let location = headers::check_not_empty(
            response.headers.get(HTTP_HEADER_LOCATION),
            "http://www.example.org",
        );

        let service_name =
            headers::check_not_empty(response.headers.get(HTTP_HEADER_USN), "undefined");

        let search_target =
            headers::check_not_empty(response.headers.get(HTTP_HEADER_ST), "undefined");

        let mut boot_id = 0u64;
        let mut config_id: Option<u64> = None;
        let mut search_port: Option<u16> = None;
        if versions.upnp.version == SpecVersion::V20.to_string() {
            boot_id = headers::check_parsed_value::<u64>(
                response
                    .headers
                    .get(HTTP_HEADER_BOOTID)
                    .unwrap_or(&"0".to_string()),
                HTTP_HEADER_BOOTID,
            )?;
            if let Some(s) = response.headers.get(HTTP_HEADER_CONFIGID) {
                config_id = s.parse::<u64>().ok();
            }
            if let Some(s) = response.headers.get(HTTP_HEADER_SEARCH_PORT) {
                search_port = s.parse::<u16>().ok();
            }
        }

        let remaining_headers: HashMap<String, String> = response
            .headers
            .clone()
            .iter()
            .filter(|(k, _)| !REQUIRED_HEADERS_V10.contains(&k.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        Ok(Response {
            max_age: Duration::from_secs(max_age),
            date,
            versions,
            location: URI::from_str(&location)
                .map_err(|_| invalid_header_value(HTTP_HEADER_LOCATION, &location))?,
            search_target: SearchTarget::from_str(&search_target)
                .map_err(|_| invalid_field_value("SearchTarget", search_target))?,
            service_name: URI::from_str(&service_name)
                .map_err(|_| invalid_field_value("URI", service_name))?,
            boot_id,
            config_id,
            search_port,
            other_headers: remaining_headers,
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl ResponseCache {
    pub fn refresh(&mut self) -> Self {
        self.to_owned()
    }

    pub fn last_updated(self) -> SystemTime {
        self.last_updated
    }

    pub fn responses(&self) -> Vec<&Response> {
        self.responses.iter().map(|r| r.response.borrow()).collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

//fn callback_wrapper(inner: &CallbackFn) -> bool {
//    false
//}
