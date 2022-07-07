use crate::error::{invalid_header_value, missing_required_header, MessageFormatError};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use tracing::error;

pub fn check_required(
    headers: &HashMap<String, String>,
    required: &[&str],
) -> Result<(), MessageFormatError> {
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
        missing_required_header(missing_headers.join(", ")).into()
    }
}

pub fn check_parsed_value<T>(header_value: &str, name: &str) -> Result<T, MessageFormatError>
where
    T: FromStr,
{
    if let Ok(v) = header_value.parse::<T>() {
        Ok(v)
    } else {
        error!(
            "check_parsed_value - header '{}', value '{}' could not be parsed",
            name, header_value
        );
        invalid_header_value(name, header_value).into()
    }
}

pub fn check_regex(
    header_value: &str,
    name: &str,
    regex: &Regex,
) -> Result<String, MessageFormatError> {
    if let Some(captured) = regex.captures(header_value) {
        Ok(captured.get(1).unwrap().as_str().to_string())
    } else {
        error!(
            "check_regex - header '{}', value '{}' did not match regex",
            name, header_value
        );
        invalid_header_value(name, header_value).into()
    }
}

pub fn check_empty(header_value: &str, name: &str) -> Result<(), MessageFormatError> {
    if header_value.trim().is_empty() {
        Ok(())
    } else {
        error!(
            "check_empty - header '{}', value '{}' should be empty",
            name, header_value
        );
        invalid_header_value(name, header_value).into()
    }
}

pub fn check_not_empty(header_entry: std::option::Option<&String>, default: &str) -> String {
    let default_value = default.to_string();
    let header_value = header_entry.unwrap_or(&default_value);
    if !header_value.trim().is_empty() {
        header_value.to_string()
    } else {
        default_value
    }
}
