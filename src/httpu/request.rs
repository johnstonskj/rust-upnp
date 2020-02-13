/*!
What's this all about then?
*/

use crate::httpu::protocol;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Request {
    pub(crate) message: String,
    pub(crate) resource: Option<String>,
    pub(crate) headers: HashMap<String, String>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Request {
    fn request_line(&self) -> String {
        format!(
            "{} {} {}/{}{}",
            self.message,
            match &self.resource {
                None => protocol::ANY_RESOURCE.to_string(),
                Some(resource) => resource.clone(),
            },
            protocol::NAME,
            protocol::VERSION,
            protocol::LINE_SEP,
        )
    }

    fn all_headers(&self) -> String {
        self.headers
            .iter()
            .map(|(k, v)| format!("{}{}{}", k, protocol::HEADER_SEP, v))
            .collect::<Vec<String>>()
            .join(protocol::LINE_SEP)
    }

    fn body(&self) -> String {
        format!("{}{}", protocol::LINE_SEP, protocol::LINE_SEP)
    }
}

impl From<&Request> for String {
    fn from(rq: &Request) -> Self {
        format!("{}{}{}", rq.request_line(), rq.all_headers(), rq.body())
    }
}
