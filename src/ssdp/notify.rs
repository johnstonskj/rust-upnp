/*!
This module provides three functions that provide 1) device available, 2) device updated, and
3) device leaving notifications over multicast UDP.
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
    pub notification_type: SearchTarget,
    pub service_name: URI,
    pub location: URL,
    pub boot_id: u32,
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

/**
When a device is added to the network, it multicasts discovery messages to advertise its root
device, any embedded devices, and any services. Each discovery message contains four major
components:

1. a potential search target (e.g., device type), sent in an `NT` (Notification Type) header,
2. a composite identifier for the advertisement, sent in a `USN` (Unique Service Name) header,
3. a URL for more information about the device (or enclosing device in the case of a service),
   sent in a `LOCATION` header, and
4. a duration for which the advertisement is valid, sent in a `CACHE-CONTROL` header.

# Parameters

* `device` - details of the device to publish as a part of the notification message. Not all device
     fields may be used in all notifications.
* `options` - protocol options such as the specification version to use and any network
     configuration values.

*/
pub fn device_available(device: &mut Device, options: Options) -> Result<(), Error> {
    let next_boot_id = device.boot_id + 1;
    let mut message_builder = RequestBuilder::new(protocol::METHOD_NOTIFY);
    message_builder
        .add_header(protocol::HEAD_HOST, protocol::MULTICAST_ADDRESS)
        .add_header(
            protocol::HEAD_CACHE_CONTROL,
            &format!("max-age={}", options.max_age),
        )
        .add_header(protocol::HEAD_LOCATION, &device.location.to_string())
        .add_header(protocol::HEAD_NT, &device.notification_type.to_string())
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

/**
When a new UPnP-enabled interface is added to a multi-homed device, the device MUST increase its
`BOOTID.UPNP.ORG` field value, multicast an `ssdp:update` message for each of the root devices,
embedded devices and embedded services to all of the existing UPnP-enabled interfaces to announce
a change in the `BOOTID.UPNP.ORG` field value, and re-advertise itself on all (existing and new)
UPnP-enabled interfaces with the new `BOOTID.UPNP.ORG` field value. Similarly, if a multi-homed
device loses connectivity on a UPnP-enabled interface and regains connectivity, or if the IP
address on one of the UPnP-enabled interfaces changes, the device MUST increase the
`BOOTID.UPNP.ORG` field value, multicast an `ssdp:update` message for each of the root devices,
embedded devices and embedded services to all the unaffected UPnP-enabled interfaces to announce a
change in the `BOOTID.UPNP.ORG` field value, and re-advertise itself on all (affected and
unaffected) UPnP-enabled interfaces with the new `BOOTID.UPNP.ORG` field value. In all cases, the
`ssdp:update` message for the root devices MUST be sent as soon as possible. Other `ssdp:update`
messages SHOULD be spread over time. However, all ssdp:update messages MUST be sent before any
announcement messages with the new `BOOTID.UPNP.ORG` field value can be sent.


When `ssdp:update` messages are sent on multiple UPnP-enabled interfaces, the messages MUST contain
identical field values except for the `HOST` and `LOCATION` field values. The `HOST` field value
of an advertisement MUST be the standard multicast address specified for the protocol (IPv4 or IPv6)
used on the interface. The URL specified in the `LOCATION` field value MUST be reachable on the
interface on which the advertisement is sent.

# Parameters

* `device` - details of the device to publish as a part of the notification message. Not all device
     fields may be used in all notifications.
* `options` - protocol options such as the specification version to use and any network
     configuration values.

*/
pub fn device_update(device: &mut Device, options: Options) -> Result<(), Error> {
    if options.spec_version == SpecVersion::V10 {
        return Err(Error::Unsupported);
    }
    let next_boot_id = device.boot_id + 1;
    let mut message_builder = RequestBuilder::new(protocol::METHOD_NOTIFY);
    message_builder
        .add_header(protocol::HEAD_HOST, protocol::MULTICAST_ADDRESS)
        .add_header(protocol::HEAD_LOCATION, &device.location.to_string())
        .add_header(protocol::HEAD_NT, &device.notification_type.to_string())
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

/**
When a device and its services are going to be removed from the network, the device SHOULD
multicast an `ssdp:byebye` message corresponding to each of the `ssdp:alive` messages it multicasted
that have not already expired. If the device is removed abruptly from the network, it might not be
possible to multicast a message. As a fallback, discovery messages MUST include an expiration
value in a `CACHE-CONTROL` field value (as explained above); if not re-advertised, the discovery
message eventually expires on its own.

When a device is about to be removed from the network, it should explicitly revoke its discovery
messages by sending one multicast request for each `ssdp:alive message` it sent. Each multicast
request must have method `NOTIFY` and `ssdp:byeby`e in the `NTS` header in the following format.

# Parameters

* `device` - details of the device to publish as a part of the notification message. Not all device
     fields may be used in all notifications.
* `options` - protocol options such as the specification version to use and any network
     configuration values.

*/
pub fn device_unavailable(device: &mut Device, options: Options) -> Result<(), Error> {
    let next_boot_id = device.boot_id + 1;
    let mut message_builder = RequestBuilder::new(protocol::METHOD_NOTIFY);
    message_builder
        .add_header(protocol::HEAD_HOST, protocol::MULTICAST_ADDRESS)
        .add_header(protocol::HEAD_NT, &device.notification_type.to_string())
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
