/*!
What's this all about then?
*/

use crate::httpu::{broadcast, Error, Options as BroadcastOptions, Request, RequestBuilder};
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
    pub search_target: SearchTarget,
    pub max_wait_time: u8,
}

#[derive(Clone, Debug)]
pub struct SearchResponse {
    max_age: u64,
    date: String,
    server_os_version: String,
    server_produce_version: String,
    location: String,
    search_target: SearchTarget,
    service_name: String,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[instrument]
pub fn search(options: SearchOptions) -> Result<Vec<SearchResponse>, Error> {
    let message: Request = RequestBuilder::new(protocol::MSG_SEARCH)
        .add_header(protocol::HEAD_HOST, protocol::MULTICAST_ADDRESS)
        .add_header(protocol::HEAD_ST, &options.search_target.to_string())
        .add_header(protocol::HEAD_MX, &format!("{}", options.max_wait_time))
        .add_header(protocol::HEAD_MAN, protocol::HTTP_EXTENSION)
        .into();

    let responses = broadcast(
        &message,
        &protocol::MULTICAST_ADDRESS.parse().unwrap(),
        &BroadcastOptions::default(),
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
            search_target: SearchTarget::RootDevices,
            max_wait_time: 2,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod protocol;
