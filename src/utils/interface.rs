use pnet::datalink;
use std::net::IpAddr;

pub fn ip_addresses(interface: String) -> Vec<IpAddr> {
    let interfaces = datalink::interfaces();
    match &interfaces.into_iter().find(|ni| ni.name == interface) {
        None => Vec::new(),
        Some(ni) => ni.ips.iter().map(|ip| ip.ip()).collect(),
    }
}

//pub fn all_names() -> Vec<String> {
//    let interfaces = datalink::interfaces();
//    interfaces
//        .iter()
//        .cloned()
//        .filter(|ni| !ni.ips.is_empty())
//        .map(|ni| ni.name)
//        .collect()
//}
//
//pub fn all_ip_addresses() -> Vec<IpAddr> {
//    let interfaces = datalink::interfaces();
//    interfaces
//        .iter()
//        .map(|ni| ni.ips.iter().map(|ip| ip.ip()))
//        .flatten()
//        .collect()
//}
