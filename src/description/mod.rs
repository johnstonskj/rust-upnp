/*!
This module implements the UPnP device and service descriptions using the UPnP template language.
*/
use crate::discovery::search::SearchTarget;
use crate::error::{invalid_value_for_type, unsupported_operation, Error};
use crate::UPNP_DOMAIN;
use std::fmt::{Display, Error as FmtError, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum TypeID {
    Device {
        domain: String,
        name: String,
        version: String,
    },
    Service {
        domain: String,
        name: String,
        version: String,
    },
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl TypeID {
    pub fn new_device(name: String, version: String) -> Self {
        TypeID::Device {
            domain: UPNP_DOMAIN.to_string(),
            name,
            version,
        }
    }

    pub fn new_device_with_domain(domain: String, name: String, version: String) -> Self {
        TypeID::Device {
            domain,
            name,
            version,
        }
    }

    pub fn new_service(name: String, version: String) -> Self {
        TypeID::Service {
            domain: UPNP_DOMAIN.to_string(),
            name,
            version,
        }
    }

    pub fn new_service_with_domain(domain: String, name: String, version: String) -> Self {
        TypeID::Service {
            domain,
            name,
            version,
        }
    }

    pub fn device_from(st: SearchTarget) -> Result<Self, Error> {
        match st {
            SearchTarget::DeviceType(type_name) => {
                let (name, version) = split_type_and_version(type_name)?;
                Ok(TypeID::new_device(name, version))
            }
            SearchTarget::DomainDeviceType(domain, type_name) => {
                let (name, version) = split_type_and_version(type_name)?;
                Ok(TypeID::new_device_with_domain(domain, name, version))
            }
            _ => unsupported_operation(st.to_string()).into(),
        }
    }

    pub fn service_from(st: SearchTarget) -> Result<Self, Error> {
        match st {
            SearchTarget::ServiceType(name) => {
                let (name, version) = split_type_and_version(name)?;
                Ok(TypeID::new_service(name, version))
            }
            SearchTarget::DomainServiceType(domain, name) => {
                let (name, version) = split_type_and_version(name)?;
                Ok(TypeID::new_service_with_domain(domain, name, version))
            }
            _ => unsupported_operation(st.to_string()).into(),
        }
    }

    pub fn default_id(&self) -> String {
        match self {
            TypeID::Device { name, .. } => format!("urn:upnp-org:deviceId:{}", name),
            TypeID::Service { name, .. } => format!("urn:upnp-org:serviceId:{}", name),
        }
    }
}

impl Display for TypeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            TypeID::Device {
                domain,
                name,
                version,
            } => write!(f, "urn:{}:device:{}:{}", domain, name, version),
            TypeID::Service {
                domain,
                name,
                version,
            } => write!(f, "urn:{}:service:{}:{}", domain, name, version),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn split_type_and_version(type_name: String) -> Result<(String, String), Error> {
    match type_name.find(':') {
        None => invalid_value_for_type("type_and_version", type_name).into(),
        Some(sep) => {
            let (name, ver) = type_name.split_at(sep);
            Ok((name.to_string(), ver.to_string()))
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod device;

pub mod service;
