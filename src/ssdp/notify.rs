use crate::httpu::{multicast_once, Error, Options as MulticastOptions, Request, RequestBuilder};
use crate::ssdp::protocol;
use crate::utils::user_agent;

#[derive(Clone, Debug)]
pub struct Device {
    service_name: String,
    location: String,
    boot_id: u64,
    config_id: u64,
    search_port: Option<u16>,
}

#[derive(Clone, Debug)]
pub struct Options {
    pub network_interface: Option<String>,
    pub max_age: Option<u16>,
    pub user_agent: Option<String>,
}

fn device_available(device: &mut Device, options: Options) -> Result<(), Error> {
    Ok(())
}

fn device_update(device: &mut Device, options: Options) -> Result<(), Error> {
    Ok(())
}

fn device_unavailable(device: &mut Device, options: Options) -> Result<(), Error> {
    let next_boot_id = device.boot_id + 1;
    let message: Request = RequestBuilder::new(protocol::METHOD_SEARCH)
        .add_header(protocol::HEAD_HOST, protocol::MULTICAST_ADDRESS)
        .add_header(protocol::HEAD_USN, &device.service_name)
        .add_header(
            protocol::HEAD_USER_AGENT,
            &user_agent::make(&options.user_agent),
        )
        .into();

    multicast_once(
        &message,
        &protocol::MULTICAST_ADDRESS.parse().unwrap(),
        &options.into(),
    )?;
    device.boot_id = next_boot_id;
    Ok(())
}

impl From<Options> for MulticastOptions {
    fn from(options: Options) -> Self {
        let mut multicast_options = MulticastOptions::default();
        multicast_options.network_interface = options.network_interface;
        multicast_options
    }
}
