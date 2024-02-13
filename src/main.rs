use std::env;

use colored::*;
use pnet::datalink; // Importa todo desde colored
use serde::{Deserialize, Serialize};
use serde_json;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "json" => json_mode(),
            _ => standard(),
        };
    }
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
        println!("{}: {}", "Nombre".green(), iface.name.yellow());
        println!("{}", "IPs:".green());
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
    for iface in all_interfaces
        .into_iter()
        .filter(|e| e.is_up() && !e.ips.is_empty())
    {
        let json = Info {
            ifname: iface.name,
            addrs: iface.ips.iter().map(|e| e.to_string()).collect(),
        };
        println!("{}", serde_json::to_string(&json).unwrap());
    }
}
