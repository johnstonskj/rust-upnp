use crate::httpu::request::Request;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RequestBuilder {
    request: Request,
}

impl RequestBuilder {
    pub fn new(message: &str) -> Self {
        RequestBuilder {
            request: Request {
                message: message.to_string(),
                resource: None,
                headers: Default::default(),
            },
        }
    }

    pub fn for_resource(&mut self, resource: &str) -> &mut Self {
        self.request.resource = Some(resource.to_string());
        self
    }

    pub fn for_any_resource(&mut self) -> &mut Self {
        self.request.resource = None;
        self
    }

    pub fn no_headers(&mut self) -> &mut Self {
        self.request.headers = Default::default();
        self
    }

    pub fn headers(&mut self, headers: HashMap<String, String>) -> &mut Self {
        self.request.headers = headers;
        self
    }

    pub fn add_header(&mut self, name: &str, value: &str) -> &mut Self {
        self.request
            .headers
            .insert(name.to_string(), value.to_string());
        self
    }
}

impl From<&mut RequestBuilder> for Request {
    fn from(rb: &mut RequestBuilder) -> Self {
        rb.request.clone()
    }
}
