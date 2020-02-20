/*!
What's this all about then?
*/

use crate::common::httpu::protocol;
use crate::{Error, MessageErrorKind};
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::from_utf8;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ResponseStatus {
    pub protocol: String,
    pub version: String,
    pub code: u16,
    pub message: String,
}

#[derive(Clone, Debug)]
pub struct Response {
    pub status: ResponseStatus,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl TryFrom<&[u8]> for Response {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let (raw_headers, body) = split_at_body(bytes);

        let headers = from_utf8(raw_headers)?;
        let mut lines = headers
            .split(protocol::LINE_SEP)
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

fn decode_status_line(line: String) -> Result<ResponseStatus, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^HTTP/([\d\.]+) (\d+) (.*)$").unwrap();
    }
    match RE.captures(&line) {
        None => {
            error!(
                "decode_status_line - could not decode status line '{}'",
                line
            );
            Err(Error::MessageFormat(
                MessageErrorKind::InvalidResponseStatus,
            ))
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
                Err(Error::MessageFormat(
                    MessageErrorKind::InvalidResponseStatus,
                ))
            }
        }
    }
}

fn decode_headers(lines: Vec<String>) -> Result<HashMap<String, String>, Error> {
    let mut headers: HashMap<String, String> = HashMap::new();
    for line in lines {
        let (key, value) = decode_header(line)?;
        headers.insert(key, value);
    }
    Ok(headers)
}

fn decode_header(line: String) -> Result<(String, String), Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-zA-Z0-9\-_]*)[ ]*:[ ]*(.*)$").unwrap();
    }
    match RE.captures(&line) {
        None => {
            error!("decode_header - could not decode header '{}'", line);
            Err(Error::MessageFormat(MessageErrorKind::InvalidHeaderFormat))
        }
        Some(captured) => Ok((
            captured.get(1).unwrap().as_str().to_uppercase(),
            captured.get(2).unwrap().as_str().to_string(),
        )),
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------
