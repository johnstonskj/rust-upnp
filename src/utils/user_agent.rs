use std::env;
use std::sync::Once;

const UA_NAME: &str = env!("CARGO_PKG_NAME");

const UA_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn make(product: &Option<String>) -> String {
    static mut PRODUCT: String = String::new();
    static mut OP_SYS: String = String::new();
    static CAPTURE: Once = Once::new();

    CAPTURE.call_once(|| unsafe {
        PRODUCT = format!("Rust-{}/{}", UA_NAME, UA_VERSION);
        OP_SYS = format!("{}/{}", os::system_name(), os::system_version());
        info!("Default User-Agent: {} UPnP/2.0 {}", OP_SYS, PRODUCT);
    });

    format!(
        "{} UPnP/2.0 {}",
        unsafe { OP_SYS.clone() },
        product.clone().unwrap_or(unsafe { PRODUCT.clone() }),
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
