/*!
What's this all about then?
*/

use std::convert::TryFrom;
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::net::{SocketAddrV4, UdpSocket};
use std::time::Duration;
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
    address: &SocketAddrV4,
    options: &Options,
) -> Result<Vec<Response>, Error> {
    let socket = UdpSocket::bind(protocol::DEFAULT_LISTENING)?;

    broadcast_using(message, address, options, &socket)
}

pub fn broadcast_using(
    message: &Request,
    address: &SocketAddrV4,
    options: &Options,
    socket: &UdpSocket,
) -> Result<Vec<Response>, Error> {
    socket.set_broadcast(true)?;
    socket.set_nonblocking(false)?;
    socket.set_read_timeout(Some(Duration::new(options.timeout, 0)))?;

    socket.connect(address)?;

    let message: String = message.into();

    event!(Level::INFO, "Message: [{}]", message);

    socket.send(message.as_bytes())?;

    let mut responses: Vec<Response> = Default::default();

    let mut buf = [0; 4096];
    match socket.recv(&mut buf) {
        Ok(received) => {
            event!(
                Level::INFO,
                received_bytes = received,
                "recv function success: {:?}",
                &buf[..received]
            );
            responses.push(Response::try_from(&buf[..received])?);
        }
        Err(e) => {
            if e.kind() != ErrorKind::WouldBlock {
                event!(Level::ERROR, "recv function failed: {:?}", e);
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
