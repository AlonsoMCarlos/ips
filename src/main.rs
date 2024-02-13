use netif;
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};
fn main() {
    // Obtener las interfaces de red
    let interfaces = netif::up().expect("Failed to get network interfaces");
    let mut ifaces = Interfaces::new();

    // Iterar por cada interfaz
    for interface in interfaces {
        // Obtener las direcciones IP de la interfaz

        ifaces.add(interface.name(), interface.address());
    }
    for iface in ifaces.ifaces.values() {
        println!("Interface: {}", iface.name);
        for ip in iface.ip_v4.iter() {
            println!("  IPv4: {}", ip);
        }
        for ip in iface.ip_v6.iter() {
            println!("  IPv6: {}", ip);
        }
    }
}

struct Interface {
    name: String,
    ip_v4: Vec<Ipv4Addr>,
    ip_v6: Vec<Ipv6Addr>,
}

struct Interfaces {
    ifaces: HashMap<String, Interface>,
}

impl Interfaces {
    fn new() -> Self {
        Self {
            ifaces: HashMap::new(),
        }
    }

    fn add(&mut self, ifname: &str, addr: &IpAddr) {
        let iface = self
            .ifaces
            .entry(ifname.to_string())
            .or_insert_with(|| Interface {
                name: ifname.to_string(),
                ip_v4: Vec::new(),
                ip_v6: Vec::new(),
            });
        match addr {
            IpAddr::V4(ip) => {
                iface.ip_v4.push(*ip);
            }
            IpAddr::V6(ip) => {
                iface.ip_v6.push(*ip);
            }
        }
    }
}
