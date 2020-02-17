/*!
This module implements the UPnP _Simple Service Discovery Protocol_ (SSDP) search and notify
interactions.
*/

use crate::{SpecVersion, UPNP_STRING};
use std::fmt::{Display, Error, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This represents a specific control point, this is optional for v1.0 and v1.1 messages
/// but `friendly_name` is required by v2.0.
///
#[derive(Clone, Debug)]
pub struct ControlPoint {
    /// Corresponds to the
    pub friendly_name: String,
    pub uuid: Option<String>,
    pub port: Option<u16>,
}

#[derive(Clone, Debug)]
pub struct ProductVersion {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug)]
pub struct ProductVersions {
    pub operating_system: ProductVersion,
    pub upnp: ProductVersion,
    pub product: ProductVersion,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for ProductVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}/{}", self.name, self.version)
    }
}

pub const fn default_product_version() -> ProductVersion {
    ProductVersion {
        name: String::new(),
        version: String::new(),
    }
}

impl ProductVersion {
    pub fn for_upnp(version: &SpecVersion) -> Self {
        ProductVersion {
            name: UPNP_STRING.to_string(),
            version: version.to_string(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ProductVersions {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{} {} {}",
            self.operating_system, self.upnp, self.product
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod search;

pub mod notify;

mod protocol;
