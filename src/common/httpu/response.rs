/*!
What's this all about then?
*/

use crate::error::{invalid_header_value, MessageFormatError};
use crate::syntax::HTTP_HEADER_LINE_SEP;
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::from_utf8;
use std::str::FromStr;
use tracing::{error, trace};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ResponseStatus {
    #[allow(dead_code)]
    protocol: String,
    #[allow(dead_code)]
    version: String,
    #[allow(dead_code)]
    code: u16,
    #[allow(dead_code)]
    message: String,
}

#[derive(Clone, Debug)]
pub struct Response {
    #[allow(dead_code)]
    status: ResponseStatus,
    pub(crate) headers: HashMap<String, String>,
    #[allow(dead_code)]
    body: Option<Vec<u8>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl TryFrom<&[u8]> for Response {
    type Error = MessageFormatError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let (raw_headers, body) = split_at_body(bytes);

        let headers = from_utf8(raw_headers)?;
        let mut lines = headers
            .split(HTTP_HEADER_LINE_SEP)
            .map(String::from)
            .collect::<Vec<String>>();

        let status = decode_status_line(lines.remove(0))?;

        let headers = decode_headers(lines)?;

        trace!("{:?}", headers);

        Ok(Response {
            status,
            headers,
            body: if body.is_empty() {
                None
            } else {
                Some(body.into())
            },
        })
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn split_at_body(all: &[u8]) -> (&[u8], &[u8]) {
    static BLANK_LINE: &[u8] = &[b'\r', b'\n', b'\r', b'\n'];
    match all
        .windows(BLANK_LINE.len())
        .position(|window| window == BLANK_LINE)
    {
        None => (all, &[]),
        Some(start) => (&all[..start], &all[start + BLANK_LINE.len()..]),
    }
}

fn decode_status_line(line: String) -> Result<ResponseStatus, MessageFormatError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^HTTP/([\d\.]+) (\d+) (.*)$").unwrap();
    }
    match RE.captures(&line) {
        None => {
            error!(
                "decode_status_line - could not decode status line '{}'",
                line
            );
            invalid_header_value("STATUS", line).into()
        }
        Some(captured) => {
            let status_code = u16::from_str(captured.get(2).unwrap().as_str()).unwrap();
            if status_code == 200 {
                Ok(ResponseStatus {
                    protocol: String::from("HTTP"),
                    version: captured.get(1).unwrap().as_str().to_string(),
                    code: status_code,
                    message: captured.get(3).unwrap().as_str().to_string(),
                })
            } else {
                error!("server returned error '{}'", status_code);
                invalid_header_value("STATUS", &status_code.to_string()).into()
            }
        }
    }
}

fn decode_headers(lines: Vec<String>) -> Result<HashMap<String, String>, MessageFormatError> {
    let mut headers: HashMap<String, String> = HashMap::new();
    for line in lines {
        let (key, value) = decode_header(line)?;
        headers.insert(key, value);
    }
    Ok(headers)
}

fn decode_header(line: String) -> Result<(String, String), MessageFormatError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-zA-Z0-9\-_]*)[ ]*:[ ]*(.*)$").unwrap();
    }
    match RE.captures(&line) {
        None => {
            error!("decode_header - could not decode header '{}'", line);
            invalid_header_value("?", line).into()
        }
        Some(captured) => Ok((
            captured.get(1).unwrap().as_str().to_uppercase(),
            captured.get(2).unwrap().as_str().to_string(),
        )),
    }
}
