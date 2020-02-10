# Crate upnp

A Rust crate providing basic Universal Plug and Play (UPnP) protocol implementations. 

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.38-green.svg)
[![crates.io](https://img.shields.io/crates/v/upnp.svg)](https://crates.io/crates/upnp)
[![docs.rs](https://docs.rs/upnp/badge.svg)](https://docs.rs/upnp)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-upnp.svg)](https://github.com/johnstonskj/rust-upnp/stargazers)

## UPnP is described in both 
[UPnP Device Architecture 1.0 ](http://www.upnp.org/specs/arch/UPnP-arch-DeviceArchitecture-v1.0.pdf) and 
[ISO/IEC 29341-1-1:2011 (ISO/IEC 29341-1-1:2011)](https://www.iso.org/standard/57494.html) UPnP Device 
Architecture — Part 1-1: UPnP Device Architecture Version 1.1 and informally discussed in 
[Universal Plug and Play — Wikipedia](https://en.wikipedia.org/wiki/Universal_Plug_and_Play).


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