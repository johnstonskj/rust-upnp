# Crate upnp

A Rust crate providing basic Universal Plug and Play (UPnP) protocol implementations. 

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.38-green.svg)
[![crates.io](https://img.shields.io/crates/v/upnp.svg)](https://crates.io/crates/upnp)
[![docs.rs](https://docs.rs/upnp/badge.svg)](https://docs.rs/upnp)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-upnp.svg)](https://github.com/johnstonskj/rust-upnp/stargazers)

> _UPnP technology allows devices to connect seamlessly and to simplify network implementation in the home and 
> corporate environments_ — [Open Connectivity Foundation](https://openconnectivity.org/developer/specifications/upnp-resources/upnp/).

UPnP is standardized in 
[ISO/IEC 29341-1-1:2011 (ISO/IEC 29341-1-1:2011)](https://www.iso.org/standard/57494.html) UPnP Device 
Architecture — Part 1-1: UPnP Device Architecture Version 1.1 and 
[UPnP Standards and Architecture](https://openconnectivity.org/developer/specifications/upnp-resources/upnp/) where
the version 2.0 standard and device types are provided in more detail.
An informal overview can be found in 
[Universal Plug and Play — Wikipedia](https://en.wikipedia.org/wiki/Universal_Plug_and_Play).

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