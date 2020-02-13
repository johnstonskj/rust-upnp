use std::env;
use std::process::Command;
use std::sync::Once;

const UA_PREFIX: &str = "Rust";

const UA_NAME: &str = env!("CARGO_PKG_NAME");

const UA_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn make(provided: &Option<String>) -> String {
    static mut AGENT: String = String::new();
    static mut COMMENTS: String = String::new();
    static CAPTURE: Once = Once::new();

    CAPTURE.call_once(|| unsafe {
        AGENT = format!("{}-{}/{}", UA_PREFIX, UA_NAME, UA_VERSION);
        COMMENTS = format!("({}; {}; {})", platform(), operating_system(), locale());
    });

    [
        provided.clone().unwrap_or_else(String::new),
        unsafe { AGENT.clone() },
        unsafe { COMMENTS.clone() },
    ]
    .join(" ")
}

#[cfg(target_os = "windows")]
#[inline]
fn platform() -> String {
    String::from("Windows")
}

#[cfg(target_os = "macos")]
#[inline]
fn platform() -> String {
    String::from("Macintosh")
}

#[cfg(all(not(target_os = "macos"), target_family = "unix"))]
#[inline]
fn platform() -> String {
    String::from("Unix")
}

#[cfg(target_family = "unix")]
#[inline]
fn operating_system() -> String {
    let uname_info = Command::new("uname")
        .arg("-sm")
        .output()
        .expect("Couldn't find `uname`");
    let uname_string = String::from_utf8(uname_info.stdout).expect("Oh crap");
    uname_string.trim().to_string()
}

#[inline]
fn locale() -> String {
    env::var("LANG").unwrap_or_else(|_| String::from("en"))
}
