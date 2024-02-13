use std::env;

use colored::*;
use pnet::datalink; // Importa todo desde colored
use serde::{Deserialize, Serialize};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "json" | "-j" | "j" => {
                if args.len() > 2 {
                    match args[2].as_str() {
                        "v4" | "4" => json_mode_v4(),
                        "v6" | "6" => json_mode_v6(),
                        _ => json_mode(),
                    }
                } else {
                    json_mode()
                }
            }

            _ => standard(),
        };
    } else {
        standard();
    };
}

fn standard() {
    // Obtiene una lista de todas las interfaces de red disponibles
    let all_interfacegs = datalink::interfaces();

    // Filtra las interfaces para obtener aquellas que están activas y tienen una dirección IP
    // Itera sobre las interfaces activas
    for iface in all_interfacegs
        .into_iter()
        .filter(|e| e.is_up() && !e.ips.is_empty())
    {
        println!("{}: {}", "IfName".green(), iface.name.yellow());
        println!("{}", "Address:".green());
        for ip in iface.ips {
            println!(" - {}", ip.to_string().blue());
        }
        println!();
    }
}

#[derive(Serialize, Deserialize)]
struct Info {
    ifname: String,
    addrs: Vec<String>,
}

fn json_mode() {
    let all_interfaces = datalink::interfaces();
    let mut info: Vec<Info> = vec![];
    for iface in all_interfaces
        .into_iter()
        .filter(|e| e.is_up() && !e.ips.is_empty())
    {
        let json = Info {
            ifname: iface.name,
            addrs: iface.ips.iter().map(|e| e.to_string()).collect(),
        };
        info.push(json);
    }
    println!("{}", serde_json::to_string(&info).unwrap());
}
// create a function to print the interface in json with only ip version 6
fn json_mode_v6() {
    let all_interfaces = datalink::interfaces();
    let mut info: Vec<Info> = vec![];
    for iface in all_interfaces
        .into_iter()
        .filter(|e| e.is_up() && !e.ips.is_empty())
    {
        let json = Info {
            ifname: iface.name,
            addrs: iface
                .ips
                .iter()
                .filter(|e| e.is_ipv6())
                .map(|e| e.to_string())
                .collect(),
        };
        info.push(json);
    }
    println!("{}", serde_json::to_string(&info).unwrap());
}

// create a function to print the interface in json with only ip version 4
fn json_mode_v4() {
    let all_interfaces = datalink::interfaces();
    let mut info: Vec<Info> = vec![];
    for iface in all_interfaces
        .into_iter()
        .filter(|e| e.is_up() && !e.ips.is_empty())
    {
        let json = Info {
            ifname: iface.name,
            addrs: iface
                .ips
                .iter()
                .filter(|e| e.is_ipv4())
                .map(|e| e.to_string())
                .collect(),
        };
        info.push(json);
    }
    println!("{}", serde_json::to_string(&info).unwrap());
}
