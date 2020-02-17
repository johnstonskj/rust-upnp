/*!
*/
use crate::httpu::{multicast_once, Options as MulticastOptions, RequestBuilder};
use crate::ssdp::search::SearchTarget;
use crate::ssdp::{protocol, ProductVersion};
use crate::utils::uri::{URI, URL};
use crate::utils::user_agent;
use crate::{Error, SpecVersion};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Device {
    pub service_name: URI,
    pub location: URL,
    pub boot_id: u64,
    pub config_id: u64,
    pub search_port: Option<u16>,
    pub secure_location: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Options {
    pub spec_version: SpecVersion,
    pub network_interface: Option<String>,
    pub max_age: u16,
    pub packet_ttl: u32,
    pub product_and_version: Option<ProductVersion>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn device_available(
    device: &mut Device,
    search_target: &SearchTarget,
    options: Options,
) -> Result<(), Error> {
    let next_boot_id = device.boot_id + 1;
    let mut message_builder = RequestBuilder::new(protocol::METHOD_NOTIFY);
    message_builder
        .add_header(protocol::HEAD_HOST, protocol::MULTICAST_ADDRESS)
        .add_header(
            protocol::HEAD_CACHE_CONTROL,
            &format!("max-age={}", options.max_age),
        )
        .add_header(protocol::HEAD_LOCATION, &device.location.to_string())
        .add_header(protocol::HEAD_NT, &search_target.to_string())
        .add_header(protocol::HEAD_NTS, protocol::NTS_ALIVE)
        .add_header(
            protocol::HEAD_SERVER,
            &user_agent::make(&options.spec_version, &options.product_and_version),
        )
        .add_header(protocol::HEAD_USN, &device.service_name.to_string());

    if options.spec_version >= SpecVersion::V11 {
        message_builder
            .add_header(protocol::HEAD_BOOTID, &device.boot_id.to_string())
            .add_header(protocol::HEAD_CONFIGID, &device.config_id.to_string());
        if let Some(search_port) = &device.search_port {
            message_builder.add_header(protocol::HEAD_SEARCH_PORT, &search_port.to_string());
        }
    }

    if options.spec_version >= SpecVersion::V20 {
        if let Some(secure_location) = &device.secure_location {
            message_builder
                .add_header(protocol::HEAD_SECURE_LOCATION, &secure_location.to_string());
        }
    }

    multicast_once(
        &message_builder.into(),
        &protocol::MULTICAST_ADDRESS.parse().unwrap(),
        &options.into(),
    )?;

    device.boot_id = next_boot_id;
    Ok(())
}

pub fn device_update(
    device: &mut Device,
    search_target: &SearchTarget,
    options: Options,
) -> Result<(), Error> {
    if options.spec_version == SpecVersion::V10 {
        return Err(Error::Unsupported);
    }
    let next_boot_id = device.boot_id + 1;
    let mut message_builder = RequestBuilder::new(protocol::METHOD_NOTIFY);
    message_builder
        .add_header(protocol::HEAD_HOST, protocol::MULTICAST_ADDRESS)
        .add_header(protocol::HEAD_LOCATION, &device.location.to_string())
        .add_header(protocol::HEAD_NT, &search_target.to_string())
        .add_header(protocol::HEAD_NTS, protocol::NTS_UPDATE)
        .add_header(protocol::HEAD_USN, &device.service_name.to_string())
        .add_header(protocol::HEAD_BOOTID, &device.boot_id.to_string())
        .add_header(protocol::HEAD_NEXT_BOOTID, &next_boot_id.to_string())
        .add_header(protocol::HEAD_CONFIGID, &device.config_id.to_string());

    if let Some(search_port) = &device.search_port {
        message_builder.add_header(protocol::HEAD_SEARCH_PORT, &search_port.to_string());
    }

    if options.spec_version >= SpecVersion::V20 {
        if let Some(secure_location) = &device.secure_location {
            message_builder
                .add_header(protocol::HEAD_SECURE_LOCATION, &secure_location.to_string());
        }
    }

    multicast_once(
        &message_builder.into(),
        &protocol::MULTICAST_ADDRESS.parse().unwrap(),
        &options.into(),
    )?;
    device.boot_id = next_boot_id;
    Ok(())
}

pub fn device_unavailable(
    device: &mut Device,
    search_target: &SearchTarget,
    options: Options,
) -> Result<(), Error> {
    let next_boot_id = device.boot_id + 1;
    let mut message_builder = RequestBuilder::new(protocol::METHOD_NOTIFY);
    message_builder
        .add_header(protocol::HEAD_HOST, protocol::MULTICAST_ADDRESS)
        .add_header(protocol::HEAD_NT, &search_target.to_string())
        .add_header(protocol::HEAD_NTS, protocol::NTS_BYE)
        .add_header(protocol::HEAD_USN, &device.service_name.to_string());

    if options.spec_version >= SpecVersion::V11 {
        message_builder
            .add_header(protocol::HEAD_BOOTID, &device.boot_id.to_string())
            .add_header(protocol::HEAD_CONFIGID, &device.config_id.to_string());
    }

    multicast_once(
        &message_builder.into(),
        &protocol::MULTICAST_ADDRESS.parse().unwrap(),
        &options.into(),
    )?;
    device.boot_id = next_boot_id;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Options {
    pub fn default_for(spec_version: SpecVersion) -> Self {
        Options {
            spec_version: spec_version.clone(),
            network_interface: None,
            max_age: 1800,
            packet_ttl: if spec_version == SpecVersion::V10 {
                4
            } else {
                2
            },
            product_and_version: None,
        }
    }
}

impl From<Options> for MulticastOptions {
    fn from(options: Options) -> Self {
        let mut multicast_options = MulticastOptions::default();
        multicast_options.network_interface = options.network_interface;
        multicast_options.packet_ttl = options.packet_ttl;
        multicast_options
    }
}
