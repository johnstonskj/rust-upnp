/*!
This module implements the _HTTP Multicast UDP_ (HTTPMU) and _HTTP Unicast UDP_ (HTTPU)  specified
components.
*/

use crate::utils::interface;
use crate::Error;
use std::convert::TryFrom;
use std::io::ErrorKind as IOErrorKind;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::time::Duration;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Options {
    pub network_interface: Option<String>,
    pub local_port: u16,
    pub timeout: u64,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

//pub fn send(message: &Request, options: &Options) -> Result<Response, Error> {}
//
//pub fn send_using(
//    message: &Request,
//    options: &Options,
//    socket: &UdpSocket,
//) -> Result<Response, Error> {
//}

#[instrument]
pub fn create_multicast_socket(options: &Options) -> Result<UdpSocket, Error> {
    let local_address = match local_address_for_interface(&options.network_interface) {
        None => SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(0, 0, 0, 0),
            options.local_port,
        )),
        Some(address) => SocketAddr::new(address, options.local_port),
    };
    info!("Binding to local address {:?}", local_address);
    let socket = UdpSocket::bind(local_address)?;

    configure_multicast_socket(&socket, options.timeout, true, true)?;
    info!(
        "Socket {:?}, read_timeout: {:?}, multicast_ttl: {}",
        socket,
        socket.read_timeout()?,
        socket.multicast_ttl_v4()?
    );

    Ok(socket)
}

#[instrument]
pub fn multicast(
    message: &Request,
    multicast_address: &SocketAddrV4,
    options: &Options,
) -> Result<Vec<Response>, Error> {
    let socket = create_multicast_socket(options)?;

    multicast_using(message, multicast_address, options, &socket)
}

#[instrument]
pub fn multicast_once(
    message: &Request,
    multicast_address: &SocketAddrV4,
    options: &Options,
) -> Result<(), Error> {
    let socket = create_multicast_socket(options)?;

    multicast_once_using(message, multicast_address, options, &socket)
}

#[instrument]
pub fn multicast_using(
    message: &Request,
    multicast_address: &SocketAddrV4,
    options: &Options,
    socket: &UdpSocket,
) -> Result<Vec<Response>, Error> {
    multicast_send_using(message, multicast_address, socket)?;

    let mut responses: Vec<Response> = Default::default();

    loop {
        let mut buf = [0u8; protocol::BUFFER_SIZE];
        info!("Waiting on discovery responses (recv_from)");
        match socket.recv_from(&mut buf) {
            Ok((received, from)) => {
                info!("received {} bytes from {:?}", received, from,);
                responses.push(Response::try_from(&buf[..received])?);
            }
            Err(e) => {
                if e.kind() == IOErrorKind::WouldBlock {
                    info!("socket timed out, no data");
                    break;
                } else {
                    error!("socket read returned error: {:?}", e);
                    return Err(Error::NetworkTransport(e.kind()));
                }
            }
        }
    }
    Ok(responses)
}

#[instrument]
pub fn multicast_once_using(
    message: &Request,
    multicast_address: &SocketAddrV4,
    options: &Options,
    socket: &UdpSocket,
) -> Result<(), Error> {
    multicast_send_using(message, multicast_address, socket)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Options {
    fn default() -> Self {
        Options {
            network_interface: None,
            local_port: 0,
            timeout: protocol::DEFAULT_TIMEOUT,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

pub fn local_address_for_interface(network_interface: &Option<String>) -> Option<IpAddr> {
    match network_interface {
        None => None,
        Some(name) => {
            let addresses = interface::ip_addresses(name.clone());
            if addresses.is_empty() {
                None
            } else {
                let address = addresses.first().unwrap();
                Some(*address)
            }
        }
    }
}

fn configure_multicast_socket(
    socket: &UdpSocket,
    timeout: u64,
    local_only: bool,

    loop_back: bool,
) -> Result<&UdpSocket, Error> {
    socket.set_nonblocking(false)?;
    socket.set_read_timeout(Some(Duration::from_secs(timeout)))?;
    if socket.local_addr().unwrap().is_ipv4() {
        socket.set_multicast_ttl_v4(if local_only { 1 } else { 10 })?;
        socket.set_multicast_loop_v4(loop_back)?;
    } else {
        socket.set_multicast_loop_v6(loop_back)?;
    }
    Ok(socket)
}

fn multicast_send_using(
    message: &Request,
    multicast_address: &SocketAddrV4,
    socket: &UdpSocket,
) -> Result<(), Error> {
    info!(
        "multicasting discovery message to address {:?} through interface {:?}",
        multicast_address,
        socket.local_addr()
    );

    let message: String = message.into();
    socket.send_to(message.as_bytes(), multicast_address)?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod builder;
pub use builder::RequestBuilder;

mod request;
pub use request::Request;

mod response;
pub use response::Response;

mod protocol;
