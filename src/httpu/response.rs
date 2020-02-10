use crate::httpu::{protocol, Error};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::net::SocketAddrV4;
use std::str::{from_utf8, Utf8Error};

#[derive(Clone, Debug)]
pub struct ResponseStatus {
    pub protocol: String,
    pub version: String,
    pub code: u16,
    pub message: String,
}

#[derive(Clone, Debug)]
pub struct Response {
    peer_address: SocketAddrV4,
    status: ResponseStatus,
    headers: HashMap<String, String>,
}

impl Response {
    pub fn peer_address(&self) -> SocketAddrV4 {
        self.peer_address
    }

    pub fn status(&self) -> ResponseStatus {
        self.status.clone()
    }

    pub fn headers_used(&self) -> Vec<String> {
        self.headers.keys().map(|v| v.clone()).collect()
    }

    pub fn header(&self, name: &str) -> Option<String> {
        self.headers.get(name).map(|v| v.clone())
    }
}

impl TryFrom<&[u8]> for Response {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let message = from_utf8(bytes)?;
        let lines = message.split(protocol::LINE_SEP);
        Err(Error::MessageFormat)
    }
}

impl From<Utf8Error> for Error {
    fn from(_: Utf8Error) -> Self {
        Error::MessageFormat
    }
}
