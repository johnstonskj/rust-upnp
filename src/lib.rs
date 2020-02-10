/*!

More Information:

* [UPnP Device Architecture 1.0 ](http://www.upnp.org/specs/arch/UPnP-arch-DeviceArchitecture-v1.0.pdf)

*/

#![warn(
    missing_debug_implementations,
    missing_docs,
    unused_extern_crates,
    rust_2018_idioms
)]

#[macro_use]
extern crate tracing;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod httpu;

pub mod ssdp;

mod scpd;

mod soap;
