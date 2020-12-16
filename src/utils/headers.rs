use crate::{Error, MessageErrorKind};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;


pub fn check_required(headers: &HashMap<String, String>, required: &[&str]) -> Result<(), Error> {
    let missing_headers: Vec<String> = required
        .iter()
        .cloned()
        .take_while(|h| !headers.contains_key(*h))
        .map(String::from)
        .collect();
    if missing_headers.is_empty() {
        Ok(())
    } else {
        error!(
            "check_required - message missing headers '{:?}'",
            missing_headers
        );
        Err(Error::MessageFormat(MessageErrorKind::MissingRequiredField))
    }
}

pub fn check_parsed_value<T>(header_value: &str, name: &str) -> Result<T, Error>
where
    T: FromStr,
{
    match header_value.parse::<T>() {
        Ok(v) => Ok(v),
        Err(_) => {
            error!(
                "check_parsed_value - header '{}', value '{}' could not be parsed",
                name, header_value
            );
            Err(Error::MessageFormat(MessageErrorKind::InvalidFieldValue))
        }
    }
}

pub fn check_regex(header_value: &str, name: &str, regex: &Regex) -> Result<String, Error> {
    match regex.captures(&header_value) {
        Some(captured) => Ok(captured.get(1).unwrap().as_str().to_string()),
        None => {
            error!(
                "check_regex - header '{}', value '{}' did not match regex",
                name, header_value
            );
            Err(Error::MessageFormat(MessageErrorKind::InvalidFieldValue))
        }
    }
}

pub fn check_empty(header_value: &str, name: &str) -> Result<(), Error> {
    if header_value.trim().is_empty() {
        Ok(())
    } else {
        error!(
            "check_empty - header '{}', value '{}' should be empty",
            name, header_value
        );
        Err(Error::MessageFormat(MessageErrorKind::InvalidFieldValue))
    }
}

pub fn check_not_empty(header_entry: std::option::Option<&String>, default: &str) -> String {
    let default_value = &default.to_string();
    let header_value = header_entry.unwrap_or(default_value);
    if !header_value.trim().is_empty() {
        return header_value.to_string();
    } else {
        return default_value.to_string();
    }
}
