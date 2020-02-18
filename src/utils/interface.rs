use pnet::datalink;
use std::net::IpAddr;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum IP {
    V4,
    V6,
}

pub fn ip_address_for_interface(
    network_interface: &Option<String>,
    network_version: &Option<IP>,
) -> Option<IpAddr> {
    match network_interface {
        None => None,
        Some(name) => {
            let addresses = ip_addresses_for_interface(name.clone(), network_version.clone());
            if addresses.is_empty() {
                None
            } else {
                let address = addresses.first().unwrap();
                Some(*address)
            }
        }
    }
}

pub fn ip_addresses_for_interface(interface: String, version: Option<IP>) -> Vec<IpAddr> {
    let interfaces = datalink::interfaces();
    match &interfaces.into_iter().find(|ni| ni.name == interface) {
        None => Vec::new(),
        Some(ni) => ni
            .ips
            .iter()
            .filter_map(|ip| match version {
                None => Some(ip.ip()),
                Some(IP::V4) => {
                    if ip.is_ipv4() {
                        Some(ip.ip())
                    } else {
                        None
                    }
                }
                Some(IP::V6) => {
                    if ip.is_ipv6() {
                        Some(ip.ip())
                    } else {
                        None
                    }
                }
            })
            .collect(),
    }
}
