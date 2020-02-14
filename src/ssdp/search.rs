use crate::httpu::{multicast, Options as MulticastOptions, RequestBuilder, Response};
use crate::ssdp::{protocol, ControlPoint};
use crate::utils::{headers, user_agent};
use crate::{Error, MessageErrorKind, SpecVersion};
use regex::Regex;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Error as FmtError, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum SearchTarget {
    All,
    RootDevices,
    Device(String),
    DeviceType(String),
    ServiceType(String),
    DomainDeviceType(String, String),
    DomainServiceType(String, String),
}

#[derive(Clone, Debug)]
pub struct SearchOptions {
    pub spec_version: SpecVersion,
    pub search_target: SearchTarget,
    pub network_interface: Option<String>,
    pub max_wait_time: u8,
    pub user_agent: Option<String>,
    pub control_point: Option<ControlPoint>,
}

#[derive(Clone, Debug)]
struct CachedResponse {
    response: SingleResponse,
    expiration: u64,
}

#[derive(Clone, Debug)]
pub struct SearchResponse {
    options: SearchOptions,
    minimum_refresh: u16,
    last_updated: u64,
    responses: Vec<CachedResponse>,
}

#[derive(Clone, Debug)]
pub struct SingleResponse {
    max_age: u64,
    date: String,
    server: String,
    location: String,
    search_target: SearchTarget,
    service_name: String,
    boot_id: u64,
    other_headers: HashMap<String, String>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[instrument]
pub fn search(
    options: SearchOptions,
    previous_response: SearchResponse,
) -> Result<SearchResponse, Error> {
    options.validate()?;
    Err(Error::MessageFormat(MessageErrorKind::VersionMismatch))
}

#[instrument]
pub fn search_once(options: SearchOptions) -> Result<Vec<SingleResponse>, Error> {
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
            &user_agent::make(&options.spec_version, &options.user_agent),
        );
    }
    // Headers added by 2.0 specification
    if options.spec_version >= SpecVersion::V20 {
        match &options.control_point {
            Some(cp) => {
                message_builder.add_header(protocol::HEAD_CP_FN, &cp.friendly_name);
                if let Some(port) = cp.port {
                    message_builder.add_header(protocol::HEAD_TCP_PORT, &port.to_string());
                }
                if let Some(uuid) = &cp.uuid {
                    message_builder.add_header(protocol::HEAD_TCP_PORT, &uuid);
                }
            }
            None => {
                error!("missing control point, required for UPnP/2.0");
                return Err(Error::MessageFormat(MessageErrorKind::MissingRequiredField));
            }
        }
    }
    println!("{:#?}", &message_builder);
    let raw_responses = multicast(
        &message_builder.into(),
        &protocol::MULTICAST_ADDRESS.parse().unwrap(),
        &options.into(),
    )?;

    let mut responses: Vec<SingleResponse> = Vec::new();
    for raw_response in raw_responses {
        responses.push(raw_response.try_into()?);
    }
    Ok(responses)
}

#[instrument]
pub fn search_device_once(options: SearchOptions) -> Result<Vec<SingleResponse>, Error> {
    options.validate()?;
    if options.spec_version >= SpecVersion::V11 {
        let mut message_builder = RequestBuilder::new(protocol::METHOD_SEARCH);
        message_builder
            .add_header(protocol::HEAD_HOST, protocol::MULTICAST_ADDRESS)
            .add_header(protocol::HEAD_MAN, protocol::HTTP_EXTENSION)
            .add_header(protocol::HEAD_ST, &options.search_target.to_string())
            .add_header(
                protocol::HEAD_USER_AGENT,
                &user_agent::make(&options.spec_version, &options.user_agent),
            );

        let raw_responses = multicast(
            &message_builder.into(),
            &protocol::MULTICAST_ADDRESS.parse().unwrap(),
            &options.into(),
        )?;

        let mut responses: Vec<SingleResponse> = Vec::new();
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

impl Default for SearchOptions {
    fn default() -> Self {
        SearchOptions {
            spec_version: SpecVersion::V10,
            network_interface: None,
            search_target: SearchTarget::RootDevices,
            max_wait_time: 2,
            user_agent: None,
            control_point: None,
        }
    }
}
impl SearchOptions {
    pub fn new(spec_version: SpecVersion) -> Self {
        let mut new = Self::default();
        new.spec_version = spec_version;
        new
    }

    pub fn validate(&self) -> Result<(), Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^$").unwrap();
        }
        if self.max_wait_time < 1 || self.max_wait_time > 120 {
            error!(
                "max_wait_time must be between 1..120 ({})",
                self.max_wait_time
            );
            return Err(Error::MessageFormat(MessageErrorKind::InvalidFieldValue));
        }
        if self.spec_version >= SpecVersion::V11 {
            if let Some(user_agent) = &self.user_agent {
                if !RE.is_match(user_agent) {
                    error!(
                        "user_agent needs to match 'ProductName/Version' ({})",
                        user_agent
                    );
                    return Err(Error::MessageFormat(MessageErrorKind::InvalidFieldValue));
                }
            }
        }
        if self.spec_version >= SpecVersion::V20 {
            if self.control_point.is_none() {
                error!("control_point required");
                return Err(Error::MessageFormat(MessageErrorKind::InvalidFieldValue));
            } else if let Some(control_point) = &self.control_point {
                if control_point.friendly_name.is_empty() {
                    error!("control_point.friendly_name required");
                    return Err(Error::MessageFormat(MessageErrorKind::InvalidFieldValue));
                }
            }
        }
        Ok(())
    }
}

impl From<SearchOptions> for MulticastOptions {
    fn from(options: SearchOptions) -> Self {
        let mut multicast_options = MulticastOptions::default();
        multicast_options.network_interface = options.network_interface;
        multicast_options.timeout = options.max_wait_time as u64;
        multicast_options
    }
}

const REQUIRED_HEADERS: [&str; 7] = [
    protocol::HEAD_BOOTID,
    protocol::HEAD_CACHE_CONTROL,
    protocol::HEAD_DATE,
    protocol::HEAD_EXT,
    protocol::HEAD_LOCATION,
    protocol::HEAD_ST,
    protocol::HEAD_USN,
];

impl TryFrom<Response> for SingleResponse {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        headers::check_required(&response.headers, &REQUIRED_HEADERS)?;
        headers::check_empty(
            response.headers.get(protocol::HEAD_EXT).unwrap(),
            protocol::HEAD_EXT,
        )?;

        let remaining_headers: HashMap<String, String> = response
            .headers
            .clone()
            .iter()
            .filter(|(k, _)| REQUIRED_HEADERS.contains(&k.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        Ok(SingleResponse {
            boot_id: headers::check_parsed_value::<u64>(
                response.headers.get(protocol::HEAD_BOOTID).unwrap(),
                protocol::HEAD_BOOTID,
            )?,
            max_age: headers::check_parsed_value::<u64>(
                &headers::check_regex(
                    response.headers.get(protocol::HEAD_CACHE_CONTROL).unwrap(),
                    protocol::HEAD_CACHE_CONTROL,
                    &Regex::new(r"max-age[ ]*=[ ]*(\d+)").unwrap(),
                )?,
                protocol::HEAD_CACHE_CONTROL,
            )?,
            date: headers::check_not_empty(
                response.headers.get(protocol::HEAD_DATE).unwrap(),
                protocol::HEAD_DATE,
            )?,
            server: headers::check_not_empty(
                response.headers.get(protocol::HEAD_SERVER).unwrap(),
                protocol::HEAD_SERVER,
            )?,
            location: headers::check_not_empty(
                response.headers.get(protocol::HEAD_LOCATION).unwrap(),
                protocol::HEAD_LOCATION,
            )?,
            search_target: SearchTarget::All,
            service_name: headers::check_not_empty(
                response.headers.get(protocol::HEAD_USN).unwrap(),
                protocol::HEAD_USN,
            )?,
            other_headers: remaining_headers,
        })
    }
}
