# Crate upnp-rs

A Rust crate providing basic Universal Plug and Play (UPnP) protocol implementations. 

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.38-green.svg)
[![crates.io](https://img.shields.io/crates/v/upnp-rs.svg)](https://crates.io/crates/upnp-rs)
[![docs.rs](https://docs.rs/upnp-rs/badge.svg)](https://docs.rs/upnp-rs)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-upnp.svg)](https://github.com/johnstonskj/rust-upnp/stargazers)

Implements the core protocols of the  _UPnP Device Architecture_ (UDA), specifically the discovery protocol for
control points to search for, and devices to notify of, device and service availability.

> _UPnP technology allows devices to connect seamlessly and to simplify network implementation in the home and 
> corporate environments_ — [Open Connectivity Foundation](https://openconnectivity.org/developer/specifications/upnp-resources/upnp/).

## Usage

Add the following to your `Cargo.toml`; currently this crate has no optional features.

```toml
upnp-rs = "0.2"
```

## API

The main client interface is the `discovery` module that provides `search` and `notify` capabilities. Over time 
the `description` module will be completed for the parsing and generation of device description messages. The 
following diagram shows the basic structure of the library with the two API modules relying on implementations of 
HTTPMU/HTTPU and SOAP respectively.

```
,--------, discover ,--------,     advertise     ,--------,
|        |--------->| disco. |<------------------|        |
| Client |          '--------'                   | Server |
|  API   | understand   :    ,--------, describe |  API   |
|        |------------------>| descr. |<---------|        |
'--------'              :    '--------'          '--------'
                        :         :
                        :         V
                        :    ,--------,
                        :    |  SOAP  |
                        :    '--------'
                        :         :
                        V         :
                    ,--------,    :
                    | HTTPMU |    :
                    '--------'    :
                        :         :
                        V         V              TCP/UDP
    ,---------------------------------------------------,
```

## Example

```rust
use upnp_rs::SpecVersion;
use upnp_rs::ssdp::search::*;

let mut options = Options::default_for(SpecVersion::V10);
options.search_target = SearchTarget::RootDevices;

match search_once(options) {
    Ok(responses) => {
        println!("search returned {} results.", responses.len());
        for (index, response) in responses.iter().enumerate() {
            println!("{}: {:#?}", index, response);
        }
    }
    Err(error) => {
        println!("search failed with error: {:#?}", error);
    }
}
```
  
## Command-Line

The command-line tool `upnp` can be used to perform basic operations using the SSDP API. Primarily these are used
for testing, but the search command can be used for general purpose discovery.

The general form of the command is _network-options command command-options_, as shown below.

```bash
upnp 0.2.0
UPnP simple qery/discovery tool.

USAGE:
    upnp [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help        Prints help information
    -6, --use-ipv6    Use IPv6 instead of the default v4
        --version     Prints version information
    -v, --verbose     The level of logging to perform, from off to trace; the default is off

OPTIONS:
        --interface <interface>          The network interface name to bind to; the default is all
    -V, --spec-version <spec-version>    The UPnP version to use, 1.0, 1.1, or 2.0; the default is 1.0

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    listen    Listen for device notifications
    search    Issue a multicast search to find devices
```

* `interface` this is the name of a local network interface such as `en0`.

``` bash
$ upnp search --help
upnp-search 0.2.0
Issue a multicast search to find devices

USAGE:
    upnp search [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --domain <domain>                  A domain to use in constructing device and service type targets; the default
                                           is the UPnP domain
    -w, --max-wait <max-wait>              The maximum wait time, in seconds, for devices to respond to multicast; the
                                           default is 2
    -s, --search-target <search-target>    The UPnP search target (all, root, device:{id}, device-type:{id}, service-
                                           type:{id}); the default is root
```

## Changes

**Version 0.2.0**

* Rewritten error handling to have more discrete errors and better reporting.
* Moved from [log](https://crates.io/crates/log) and
  [env_logger](https://crates.io/crates/env_logger) to
  [tracing](https://crates.io/crates/tracing) and
  [tracing_subscriber](https://crates.io/crates/tracing_subscriber).
* Upgraded dependencies, especially [pnet](https://crates.io/crates/pnet)
  which had a reported vulnerability.
* Moved all *protocol* constants into the `syntax` module.
* Using crate `os_version` for platform detection.
* Added build file and adsjusted workflow to get a clean Windows build.

**Version 0.1.0**

* Ability to issue multicast non-caching search command, with parsed results.
* Ability to send multicast device notifications.
* Command-line tool `upnp` to issue test commands.

## TODO

1. Finish parsing search results.
2. Support listening for notifications.
3. Support fetching device details.
    4. Support for sending notifications.
