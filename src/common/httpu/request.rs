/*!
What's this all about then?
*/

use crate::syntax::{
    HTTP_HEADER_LINE_SEP, HTTP_HEADER_SEP, HTTP_MATCH_ANY_RESOURCE, HTTP_PROTOCOL_NAME,
    HTTP_PROTOCOL_VERSION,
};
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
                None => HTTP_MATCH_ANY_RESOURCE.to_string(),
                Some(resource) => resource.clone(),
            },
            HTTP_PROTOCOL_NAME,
            HTTP_PROTOCOL_VERSION,
            HTTP_HEADER_LINE_SEP,
        )
    }

    fn all_headers(&self) -> String {
        self.headers
            .iter()
            .map(|(k, v)| format!("{}{}{}", k, HTTP_HEADER_SEP, v))
            .collect::<Vec<String>>()
            .join(HTTP_HEADER_LINE_SEP)
    }

    fn body(&self) -> String {
        format!("{}{}", HTTP_HEADER_LINE_SEP, HTTP_HEADER_LINE_SEP)
    }
}

impl From<&Request> for String {
    fn from(rq: &Request) -> Self {
        format!("{}{}{}", rq.request_line(), rq.all_headers(), rq.body())
    }
}
