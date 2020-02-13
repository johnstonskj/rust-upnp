/*!
What's this all about then?
*/

use crate::httpu::{broadcast, Error, Options as BroadcastOptions, Request, RequestBuilder};
use crate::utils::user_agent;
use std::collections::HashMap;
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
    // add min time between updates
}

#[derive(Clone, Debug)]
struct CachedResponse {
    response: SingleResponse,
    expiration: u64,
}

#[derive(Clone, Debug)]
pub struct SearchResponse {
    options: SearchOptions,
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
pub fn search(options: SearchOptions) -> Result<Vec<SingleResponse>, Error> {
    let message: Request = RequestBuilder::new(protocol::MSG_SEARCH)
        .add_header(protocol::HEAD_HOST, protocol::MULTICAST_ADDRESS)
        .add_header(protocol::HEAD_ST, &options.search_target.to_string())
        .add_header(protocol::HEAD_MX, &format!("{}", options.max_wait_time))
        .add_header(protocol::HEAD_MAN, protocol::HTTP_EXTENSION)
        .add_header(
            protocol::HEAD_USER_AGENT,
            &user_agent::make(&options.user_agent),
        )
        .into();

    println!("{:#?}", message);

    let responses = broadcast(
        &message,
        &protocol::MULTICAST_ADDRESS.parse().unwrap(),
        &options.into(),
    )?;
    Ok(Vec::new())
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
        }
    }
}

impl From<SearchOptions> for BroadcastOptions {
    fn from(options: SearchOptions) -> Self {
        let mut broadcast_options = BroadcastOptions::default();
        broadcast_options.network_interface = options.network_interface;
        broadcast_options.timeout = options.max_wait_time as u64;
        broadcast_options
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod protocol;
