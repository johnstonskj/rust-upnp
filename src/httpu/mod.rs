/*!
This module implements the _HTTP Multicast UDP_ (HTTPMU) and _HTTP Unicast UDP_ (HTTPU)  specified
components.
*/

use crate::utils::interface;
use crate::Error;
use std::convert::TryFrom;
use std::io::ErrorKind as IOErrorKind;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket};
use std::time::Duration;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Options {
    pub network_interface: Option<String>,
    pub network_version: Option<IP>,
    pub local_port: u16,
    pub recv_timeout: u64,
    pub packet_ttl: u32,
    pub local_network_only: bool,
    pub loop_back_also: bool,
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

pub fn create_multicast_socket(
    to_address: &SocketAddr,
    options: &Options,
) -> Result<UdpSocket, Error> {
    debug!("create_multicast_socket - options: {:?}", options);
    let local_address = match interface::ip_address_for_interface(
        &options.network_interface,
        &options.network_version,
    ) {
        None => match &options.network_version {
            Some(IP::V6) => SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
                options.local_port,
                0,
                0,
            )),
            _ => SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::new(0, 0, 0, 0),
                options.local_port,
            )),
        },
        Some(address) => SocketAddr::new(address, options.local_port),
    };
    trace!(
        "create_multicast_socket - binding to local_address: {:?}",
        local_address
    );
    let socket = UdpSocket::bind(local_address)?;

    trace!("create_multicast_socket - setting socket options");
    socket.set_nonblocking(false)?;
    socket.set_ttl(options.packet_ttl)?;
    socket.set_read_timeout(Some(Duration::from_secs(options.recv_timeout)))?;
    match (to_address, local_address) {
        (SocketAddr::V4(to_address), SocketAddr::V4(local_address)) => {
            socket.join_multicast_v4(to_address.ip(), &local_address.ip())?;
            socket.set_multicast_ttl_v4(if options.local_network_only { 1 } else { 10 })?;
            socket.set_multicast_loop_v4(options.loop_back_also)?;
        }
        (SocketAddr::V6(_), SocketAddr::V6(_)) => {
            socket.set_multicast_loop_v6(options.loop_back_also)?;
        }
        _ => return Err(Error::Unsupported),
    }

    trace!(
        "create_multicast_socket - socket: {:?}, read_timeout: {:?}, ttl: {:?}, multicast_ttl: {}",
        socket,
        socket.read_timeout()?,
        socket.ttl()?,
        socket.multicast_ttl_v4()?
    );

    Ok(socket)
}

pub fn multicast(
    message: &Request,
    to_address: &SocketAddr,
    options: &Options,
) -> Result<Vec<Response>, Error> {
    let socket = create_multicast_socket(to_address, options)?;

    multicast_using(message, to_address, &socket)
}

pub fn multicast_once(
    message: &Request,
    to_address: &SocketAddr,
    options: &Options,
) -> Result<(), Error> {
    let socket = create_multicast_socket(to_address, options)?;

    multicast_once_using(message, to_address, &socket)
}

pub fn multicast_using(
    message: &Request,
    to_address: &SocketAddr,
    socket: &UdpSocket,
) -> Result<Vec<Response>, Error> {
    multicast_send_using(message, to_address, socket)?;

    let mut responses: Vec<Response> = Default::default();

    loop {
        let mut buf = [0u8; protocol::BUFFER_SIZE];
        trace!(
            "multicast_using - blocking on recv_from, buffer size {}",
            protocol::BUFFER_SIZE
        );
        match socket.recv_from(&mut buf) {
            Ok((received, from)) => {
                trace!(
                    "multicast_using - received {} bytes from {:?}",
                    received,
                    from,
                );
                responses.push(Response::try_from(&buf[..received])?);
            }
            Err(e) => {
                if e.kind() == IOErrorKind::WouldBlock {
                    trace!("multicast_using - socket timed out, no data");
                    break;
                } else {
                    error!("multicast_using - socket read returned error: {:?}", e);
                    return Err(Error::NetworkTransport(e.kind()));
                }
            }
        }
    }
    Ok(responses)
}

pub fn multicast_once_using(
    message: &Request,
    to_address: &SocketAddr,
    socket: &UdpSocket,
) -> Result<(), Error> {
    multicast_send_using(message, to_address, socket)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Options {
    fn default() -> Self {
        Options {
            network_interface: None,
            network_version: None,
            local_port: 0,
            recv_timeout: protocol::DEFAULT_TIMEOUT,
            packet_ttl: 2,
            local_network_only: false,
            loop_back_also: false,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
fn multicast_send_using(
    message: &Request,
    to_address: &SocketAddr,
    socket: &UdpSocket,
) -> Result<(), Error> {
    let message: String = message.into();
    socket.send_to(message.as_bytes(), to_address)?;
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
use crate::utils::interface::IP;
pub use response::Response;

mod protocol;
