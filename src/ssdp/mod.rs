/*!
This module implements the UPnP _Simple Service Discovery Protocol_ (SSDP) search and notify
interactions.
*/

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

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod search;

pub mod notify;

mod protocol;
