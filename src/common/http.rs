use crate::error::{unsupported_operation, Error};
use reqwest::blocking::Client;
use tracing::info;

pub fn fetch<T>(url: String) -> Result<T, Error> {
    let client = Client::new();
    fetch_with(url, &client)
}

pub fn fetch_with<T>(url: String, client: &Client) -> Result<T, Error> {
    info!("fetch_with - fetching {}", url);
    let response = client.get(&url).send()?;
    info!("fetch_with - received {:?}", &response);
    unsupported_operation(url).into()
}
