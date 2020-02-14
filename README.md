# Crate upnp

A Rust crate providing basic Universal Plug and Play (UPnP) protocol implementations. 

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.38-green.svg)
[![crates.io](https://img.shields.io/crates/v/upnp.svg)](https://crates.io/crates/upnp)
[![docs.rs](https://docs.rs/upnp/badge.svg)](https://docs.rs/upnp)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-upnp.svg)](https://github.com/johnstonskj/rust-upnp/stargazers)

> _UPnP technology allows devices to connect seamlessly and to simplify network implementation in the home and 
> corporate environments_ — [Open Connectivity Foundation](https://openconnectivity.org/developer/specifications/upnp-resources/upnp/).

## API

TBD

```
,--------, discover ,--------,     advertise     ,--------,
|        |--------->|  SSDP  |<------------------|        |
| Client |          '--------'                   | Server |
|  API   | understand   :    ,--------, describe |  API   |
|        |------------------>|  SCPD  |<---------|        |
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

## Command-Line

TBD

## Changes

**Version 0.1.0**

* Initial commit stream to Github from private project.
* Ability to issue search command, only receive raw results.
* Command-line tool `upnp` to issue test commands.

## TODO

1. Finish parsing search results.
2. Support listening for notifications.
3. Support fetching device details.
4. Support for sending notifications.