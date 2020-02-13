use crate::httpu::{
    multicast, Error, Options as MulticastOptions, Request, RequestBuilder, Response,
};
use crate::ssdp::protocol;
use crate::utils::{headers, user_agent};
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
    pub network_interface: Option<String>,
    pub search_target: SearchTarget,
    pub max_wait_time: u8,
    pub user_agent: Option<String>,
    pub minimum_refresh: u16,
}

#[derive(Clone, Debug)]
struct CachedResponse {
    response: SingleResponse,
    expiration: u64,
}

#[derive(Clone, Debug)]
pub struct SearchResponse {
    options: SearchOptions,
    last_updated: u64,
    responses: Vec<CachedResponse>,
}

#[derive(Clone, Debug)]
pub struct SingleResponse {
    max_age: u64,
    date: String,
    server_os_version: String,
    server_produce_version: String,
    location: String,
    search_target: SearchTarget,
    service_name: String,
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
    Err(Error::MessageFormat)
}

#[instrument]
pub fn search_once(options: SearchOptions) -> Result<Vec<SingleResponse>, Error> {
    let message: Request = RequestBuilder::new(protocol::METHOD_SEARCH)
        .add_header(protocol::HEAD_HOST, protocol::MULTICAST_ADDRESS)
        .add_header(protocol::HEAD_ST, &options.search_target.to_string())
        .add_header(protocol::HEAD_MX, &format!("{}", options.max_wait_time))
        .add_header(protocol::HEAD_MAN, protocol::HTTP_EXTENSION)
        .add_header(
            protocol::HEAD_USER_AGENT,
            &user_agent::make(&options.user_agent),
        )
        .into();

    let raw_responses = multicast(
        &message,
        &protocol::MULTICAST_ADDRESS.parse().unwrap(),
        &options.into(),
    )?;

    let mut responses: Vec<SingleResponse> = Vec::new();
    for raw_response in raw_responses {
        responses.push(raw_response.try_into()?);
    }
    Ok(responses)
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
            network_interface: None,
            search_target: SearchTarget::RootDevices,
            max_wait_time: 2,
            user_agent: None,
            minimum_refresh: 0,
        }
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
        let boot_id = headers::check_parsed_value::<u64>(
            response.headers.get(protocol::HEAD_BOOTID).unwrap(),
            protocol::HEAD_BOOTID,
        )?;

        Err(Error::MessageFormat)
    }
}
