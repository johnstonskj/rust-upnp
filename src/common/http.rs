use crate::Error;
use reqwest::{blocking::Client, Error as HTTPError};

pub fn fetch<T>(url: String) -> Result<T, Error> {
    let client = Client::new();
    fetch_with(url, &client)
}

pub fn fetch_with<T>(url: String, client: &Client) -> Result<T, Error> {
    info!("fetch_with - fetching {}", url);
    let response = client.get(&url).send()?;
    info!("fetch_with - received {:?}", &response);
    Err(Error::Unsupported)
}

impl From<HTTPError> for Error {
    fn from(e: HTTPError) -> Self {
        error!("HTTP error: {:?}", e);
        Error::Messaging
    }
}
