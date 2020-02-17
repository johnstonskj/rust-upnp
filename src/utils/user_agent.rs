use crate::ssdp::{default_product_version, ProductVersion};
use crate::SpecVersion;
use std::sync::Once;

const UA_NAME: &str = env!("CARGO_PKG_NAME");

const UA_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn make(spec_version: &SpecVersion, product: &Option<ProductVersion>) -> String {
    static mut PRODUCT: ProductVersion = default_product_version();
    static mut OP_SYS: ProductVersion = default_product_version();
    static CAPTURE: Once = Once::new();

    CAPTURE.call_once(|| unsafe {
        PRODUCT = ProductVersion {
            name: format!("Rust-{}", UA_NAME),
            version: UA_VERSION.to_string(),
        };
        OP_SYS = ProductVersion {
            name: os::system_name(),
            version: os::system_version(),
        };
        trace!("make - default User-Agent: {} UPnP/? {}", OP_SYS, PRODUCT);
    });

    format!(
        "{} {} {}",
        unsafe { &OP_SYS },
        ProductVersion::for_upnp(spec_version),
        product.as_ref().unwrap_or(unsafe { &PRODUCT }),
    )
}

#[cfg(target_os = "macos")]
mod os {
    use std::process::Command;

    #[inline]
    pub fn system_name() -> String {
        let cmd_output = Command::new("sw_vers")
            .arg("-productName")
            .output()
            .expect("Couldn't find `sw_vers`");
        let output_string = String::from_utf8(cmd_output.stdout).expect("Oh crap");
        output_string.trim().to_string()
    }

    #[inline]
    pub fn system_version() -> String {
        let cmd_output = Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .expect("Couldn't find `sw_vers`");
        let output_string = String::from_utf8(cmd_output.stdout).expect("Oh crap");
        output_string.trim().to_string()
    }
}

#[cfg(all(not(target_os = "macos"), target_family = "unix"))]
mod os {
    use std::process::Command;

    #[inline]
    pub fn system_name() -> String {
        let cmd_output = Command::new("uname")
            .arg("-o")
            .output()
            .expect("Couldn't find `uname`");
        let output_string = String::from_utf8(cmd_output.stdout).expect("Oh crap");
        output_string.trim().to_string()
    }

    #[inline]
    pub fn system_version() -> String {
        let cmd_output = Command::new("uname")
            .arg("-r")
            .output()
            .expect("Couldn't find `uname`");
        let output_string = String::from_utf8(cmd_output.stdout).expect("Oh crap");
        output_string.trim().to_string()
    }
}
