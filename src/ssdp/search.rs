/*!
This module provides three functions that provide 1) multicast search, 2) unicast search, and 3)
multicast search with caching. The caching version of search will merge the set of new responses
with any (non-expired) previously cached responses.

*/
use crate::httpu::{
    multicast, Options as MulticastOptions, RequestBuilder, Response as MulticastResponse,
};
use crate::ssdp::{protocol, ControlPoint, ProductVersion, ProductVersions};
use crate::utils::uri::{URI, URL};
use crate::utils::{headers, user_agent};
use crate::{Error, MessageErrorKind, SpecVersion};
use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Error as FmtError, Formatter};
use std::net::SocketAddrV4;
use std::str::FromStr;
use std::time::{Duration, SystemTime};

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
    RootDevices,
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
/// This type encapsulates a set of mostly optional values to be used to construct messages to
/// send.
///
/// As such `Options::default()` is usually sufficient, in cases where a client wishes to select
/// a specific version of the specification use `Options::new`. Currently the only time a value
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
    expiration: SystemTime,
}

#[derive(Clone, Debug)]
pub struct ResponseCache {
    options: Options,
    minimum_refresh: Duration,
    last_updated: SystemTime,
    responses: Vec<CachedResponse>,
}

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
pub fn search(options: Options) -> Result<ResponseCache, Error> {
    info!("search - options: {:?}", options);
    options.validate()?;
    Err(Error::MessageFormat(MessageErrorKind::VersionMismatch))
}

///
/// Perform a multicast search but return the results immediately as a vector, not wrapped
/// in a cache.
///
/// The search function can be configured using the [`Options`](struct.Options.html) struct,
/// although the defaults are reasonable for most clients.
///
pub fn search_once(options: Options) -> Result<Vec<Response>, Error> {
    info!("search_once - options: {:?}", options);
    options.validate()?;
    let mut message_builder = RequestBuilder::new(protocol::METHOD_SEARCH);
    // All headers from the original 1.0 specification.
    message_builder
        .add_header(protocol::HEAD_HOST, protocol::MULTICAST_ADDRESS)
        .add_header(protocol::HEAD_MAN, protocol::HTTP_EXTENSION)
        .add_header(protocol::HEAD_MX, &format!("{}", options.max_wait_time))
        .add_header(protocol::HEAD_ST, &options.search_target.to_string());
    // Headers added by 1.1 specification
    if options.spec_version >= SpecVersion::V11 {
        message_builder.add_header(
            protocol::HEAD_USER_AGENT,
            &user_agent::make(&options.spec_version, &options.product_and_version),
        );
    }
    // Headers added by 2.0 specification
    if options.spec_version >= SpecVersion::V20 {
        match &options.control_point {
            Some(cp) => {
                message_builder.add_header(protocol::HEAD_CP_FN, &cp.friendly_name);
                if let Some(uuid) = &cp.uuid {
                    message_builder.add_header(protocol::HEAD_CP_UUID, &uuid);
                }
                if let Some(port) = cp.port {
                    message_builder.add_header(protocol::HEAD_TCP_PORT, &port.to_string());
                }
            }
            None => {
                error!("search_once - missing control point, required for UPnP/2.0");
                return Err(Error::MessageFormat(MessageErrorKind::MissingRequiredField));
            }
        }
    }
    trace!("search_once - {:?}", &message_builder);
    let raw_responses = multicast(
        &message_builder.into(),
        &protocol::MULTICAST_ADDRESS.parse().unwrap(),
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
pub fn search_once_to_device(
    options: Options,
    device_address: SocketAddrV4,
) -> Result<Vec<Response>, Error> {
    info!(
        "search_once_to_device - options: {:?}, device_address: {:?}",
        options, device_address
    );
    options.validate()?;
    if options.spec_version >= SpecVersion::V11 {
        let mut message_builder = RequestBuilder::new(protocol::METHOD_SEARCH);
        message_builder
            .add_header(protocol::HEAD_HOST, protocol::MULTICAST_ADDRESS)
            .add_header(protocol::HEAD_MAN, protocol::HTTP_EXTENSION)
            .add_header(protocol::HEAD_ST, &options.search_target.to_string())
            .add_header(
                protocol::HEAD_USER_AGENT,
                &user_agent::make(&options.spec_version, &options.product_and_version),
            );

        let raw_responses = multicast(&message_builder.into(), &device_address, &options.into())?;

        let mut responses: Vec<Response> = Vec::new();
        for raw_response in raw_responses {
            responses.push(raw_response.try_into()?);
        }
        Ok(responses)
    } else {
        Err(Error::Unsupported)
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
                SearchTarget::RootDevices => "upnp:rootdevice".to_string(),
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref DOMAIN_URN: Regex =
                Regex::new(r"^urn:([^:]+):(device|service):(.+)$").unwrap();
        }
        if s == "ssdp::all" {
            Ok(SearchTarget::All)
        } else if s == "upnp:rootdevice" {
            Ok(SearchTarget::RootDevices)
        } else if s.starts_with("uuid:") {
            Ok(SearchTarget::Device(s[5..].to_string()))
        } else if s.starts_with("urn:schemas-upnp-org:device:") {
            Ok(SearchTarget::DeviceType(s[28..].to_string()))
        } else if s.starts_with("urn:schemas-upnp-org:service:") {
            Ok(SearchTarget::ServiceType(s[29..].to_string()))
        } else if s.starts_with("urn:") {
            match DOMAIN_URN.captures(s) {
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
                    Err(())
                }
            }
        } else {
            error!("Could not parse '{}' as a search target", s);
            Err(())
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Options {
    pub fn default_for(spec_version: SpecVersion) -> Self {
        Options {
            spec_version: spec_version.clone(),
            network_interface: None,
            search_target: SearchTarget::RootDevices,
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

    pub fn for_control_point(control_point: ControlPoint) -> Self {
        let mut new = Self::default_for(SpecVersion::V20);
        new.control_point = Some(control_point.clone());
        new
    }

    pub fn validate(&self) -> Result<(), Error> {
        lazy_static! {
            static ref UA_VERSION: Regex = Regex::new(r"^[\d\.]+$").unwrap();
        }
        if self.max_wait_time < 1 || self.max_wait_time > 120 {
            error!(
                "validate - max_wait_time must be between 1..120 ({})",
                self.max_wait_time
            );
            return Err(Error::MessageFormat(MessageErrorKind::InvalidFieldValue));
        }
        if self.spec_version >= SpecVersion::V11 {
            if let Some(user_agent) = &self.product_and_version {
                if user_agent.name.contains('/') || !UA_VERSION.is_match(&user_agent.version) {
                    error!(
                        "validate - user_agent needs to match 'ProductName/Version' ({:?})",
                        user_agent
                    );
                    return Err(Error::MessageFormat(MessageErrorKind::InvalidFieldValue));
                }
            }
        }
        if self.spec_version >= SpecVersion::V20 {
            if self.control_point.is_none() {
                error!("validate - control_point required");
                return Err(Error::MessageFormat(MessageErrorKind::InvalidFieldValue));
            } else if let Some(control_point) = &self.control_point {
                if control_point.friendly_name.is_empty() {
                    error!("validate - control_point.friendly_name required");
                    return Err(Error::MessageFormat(MessageErrorKind::InvalidFieldValue));
                }
            }
        }
        Ok(())
    }
}

impl From<Options> for MulticastOptions {
    fn from(options: Options) -> Self {
        let mut multicast_options = MulticastOptions::default();
        multicast_options.network_interface = options.network_interface;
        multicast_options.packet_ttl = options.packet_ttl;
        multicast_options.recv_timeout = options.max_wait_time as u64;
        multicast_options
    }
}

// ------------------------------------------------------------------------------------------------

const REQUIRED_HEADERS_V10: [&str; 7] = [
    protocol::HEAD_CACHE_CONTROL,
    protocol::HEAD_DATE,
    protocol::HEAD_EXT,
    protocol::HEAD_LOCATION,
    protocol::HEAD_SERVER,
    protocol::HEAD_ST,
    protocol::HEAD_USN,
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
            response.headers.get(protocol::HEAD_EXT).unwrap(),
            protocol::HEAD_EXT,
        )?;

        let server = response.headers.get(protocol::HEAD_SERVER).unwrap();
        let versions = match UA_ALL.captures(response.headers.get(protocol::HEAD_SERVER).unwrap()) {
            Some(captures) => ProductVersions {
                operating_system: ProductVersion {
                    name: captures.get(1).unwrap().as_str().to_string(),
                    version: captures.get(2).unwrap().as_str().to_string(),
                },
                upnp: ProductVersion {
                    name: captures.get(3).unwrap().as_str().to_string(),
                    version: captures.get(4).unwrap().as_str().to_string(),
                },
                product: ProductVersion {
                    name: captures.get(5).unwrap().as_str().to_string(),
                    version: captures.get(6).unwrap().as_str().to_string(),
                },
            },
            None => {
                error!("invalid value for server header '{}", server);
                return Err(Error::MessageFormat(MessageErrorKind::InvalidFieldValue));
            }
        };

        let max_age = headers::check_parsed_value::<u64>(
            &headers::check_regex(
                response.headers.get(protocol::HEAD_CACHE_CONTROL).unwrap(),
                protocol::HEAD_CACHE_CONTROL,
                &Regex::new(r"max\-age[ ]*=[ ]*(\d+)").unwrap(),
            )?,
            protocol::HEAD_CACHE_CONTROL,
        )?;

        let date = headers::check_not_empty(
            response.headers.get(protocol::HEAD_DATE).unwrap(),
            protocol::HEAD_DATE,
        )?;

        let location = headers::check_not_empty(
            response.headers.get(protocol::HEAD_LOCATION).unwrap(),
            protocol::HEAD_LOCATION,
        )?;

        let service_name = headers::check_not_empty(
            response.headers.get(protocol::HEAD_USN).unwrap(),
            protocol::HEAD_USN,
        )?;

        let search_target = headers::check_not_empty(
            response.headers.get(protocol::HEAD_ST).unwrap(),
            protocol::HEAD_ST,
        )?;

        let mut boot_id = 0u64;
        let mut config_id: Option<u64> = None;
        let mut search_port: Option<u16> = None;
        if versions.upnp.version == SpecVersion::V20.to_string() {
            boot_id = headers::check_parsed_value::<u64>(
                response
                    .headers
                    .get(protocol::HEAD_BOOTID)
                    .unwrap_or(&"0".to_string()),
                protocol::HEAD_BOOTID,
            )?;
            if let Some(s) = response.headers.get(protocol::HEAD_CONFIGID) {
                config_id = s.parse::<u64>().ok();
            }
            if let Some(s) = response.headers.get(protocol::HEAD_SEARCH_PORT) {
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
                .map_err(|_| Error::MessageFormat(MessageErrorKind::InvalidFieldValue))?,
            search_target: SearchTarget::from_str(&search_target)
                .map_err(|_| Error::MessageFormat(MessageErrorKind::InvalidFieldValue))?,
            service_name: URI::from_str(&service_name)
                .map_err(|_| Error::MessageFormat(MessageErrorKind::InvalidFieldValue))?,
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
