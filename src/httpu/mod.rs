/*!
What's this all about then?
*/

use crate::utils::interface;
use std::convert::TryFrom;
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::time::{Duration, SystemTime};
use tracing::Level;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Error {
    NetworkTransport(ErrorKind),
    MessageFormat,
}

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

pub fn broadcast(
    message: &Request,
    broadcast_address: &SocketAddrV4,
    options: &Options,
) -> Result<Vec<Response>, Error> {
    let local_address = match local_address_for_interface(&options.network_interface) {
        None => SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(0, 0, 0, 0),
            options.local_port,
        )),
        Some(address) => SocketAddr::new(address, options.local_port),
    };
    let socket = UdpSocket::bind(local_address)?;

    broadcast_using(message, broadcast_address, options, &socket)
}

pub fn broadcast_using(
    message: &Request,
    broadcast_address: &SocketAddrV4,
    options: &Options,
    socket: &UdpSocket,
) -> Result<Vec<Response>, Error> {
    configure_broadcast_socket(socket, options.timeout, true, true)?;

    let message: String = message.into();
    info!(
        "Sending discovery message to address {:?} on interface {:?}",
        broadcast_address,
        socket.local_addr()
    );
    socket.send_to(message.as_bytes(), broadcast_address)?;

    let mut responses: Vec<Response> = Default::default();

    let now = SystemTime::now();
    let mut buf = [0; 4096];
    info!("Waiting on discovery responses (recv_from)");
    match socket.recv_from(&mut buf) {
        Ok((received, from)) => {
            event!(
                Level::INFO,
                received_bytes = received,
                "received {} bytes from {:?}, data: {:?}",
                received,
                from,
                &buf[..received]
            );
            responses.push(Response::try_from(&buf[..received])?);
        }
        Err(e) => {
            warn!("Error: {:?} @ {:?}", e, now.elapsed());
            if e.kind() != ErrorKind::WouldBlock {
                event!(Level::ERROR, "recv function returned error: {:?}", e);
                return Err(Error::NetworkTransport(e.kind()));
            }
        }
    }

    Ok(responses)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<IOError> for Error {
    fn from(e: IOError) -> Self {
        Error::NetworkTransport(e.kind())
    }
}

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
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

pub fn local_address_for_interface(network_interface: &Option<String>) -> Option<IpAddr> {
    match network_interface {
        None => None,
        Some(name) => {
            info!("Looking up address for interface {}", name);

            let addresses = interface::ip_addresses(name.clone());
            if addresses.is_empty() {
                None
            } else {
                let address = addresses.first().unwrap();
                Some(address.clone())
            }
        }
    }
}

fn configure_broadcast_socket(
    socket: &UdpSocket,
    timeout: u64,
    local_only: bool,
    loop_back: bool,
) -> Result<&UdpSocket, Error> {
    event!(Level::INFO, "Setting socket options...");
    socket.set_broadcast(true)?;
    socket.set_nonblocking(false)?;
    socket.set_read_timeout(Some(Duration::from_secs(timeout)))?;
    if socket.local_addr().unwrap().is_ipv4() {
        socket.set_multicast_ttl_v4(if local_only { 1 } else { 10 })?;
        socket.set_multicast_loop_v4(loop_back)?;
    } else {
        socket.set_multicast_loop_v6(loop_back)?;
    }
    event!(
        Level::INFO,
        "... {:?}, broadcast: {}, read_timeout: {:?}, multicast_ttl: {}",
        socket,
        socket.broadcast()?,
        socket.read_timeout()?,
        socket.multicast_ttl_v4()?
    );
    Ok(socket)
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
