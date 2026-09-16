use std::collections::HashSet;
use std::env;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};

/// DNS seeds públicos de la red principal de Bitcoin.
const SEEDS: &[&str] = &[
    "seed.bitcoin.sipa.be",
    "dnsseed.bluematt.me",
    "seed.bitcoin.jonasschnelli.ch",
    "seed.btc.petertodd.net",
    "seed.bitcoin.sprovoost.nl",
    "dnsseed.emzy.de",
    "seed.bitcoin.wiz.biz",
    "seed.mainnet.achownodes.xyz",
];

const SEPARATOR: &str = "-------------------------------------------------------";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AddressKind {
    Ipv4,
    Ipv6,
}

impl std::fmt::Display for AddressKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ipv4 => write!(formatter, "IPv4"),
            Self::Ipv6 => write!(formatter, "IPv6"),
        }
    }
}

/// Resuelve una semilla y etiqueta cada dirección por su familia IP.
fn resolve(seed: &str) -> Vec<(IpAddr, AddressKind)> {
    let Ok(results) = (seed, 0).to_socket_addrs() else {
        return Vec::new();
    };

    let mut addresses = Vec::new();
    for address in results {
        let resolved = match address {
            SocketAddr::V4(address) => (IpAddr::V4(*address.ip()), AddressKind::Ipv4),
            SocketAddr::V6(address) => (IpAddr::V6(*address.ip()), AddressKind::Ipv6),
        };
        if !addresses.contains(&resolved) {
            addresses.push(resolved);
        }
    }

    addresses
}

fn main() {
    let provided_seeds: Vec<String> = env::args().skip(1).collect();
    let seeds: Vec<&str> = if provided_seeds.is_empty() {
        SEEDS.to_vec()
    } else {
        provided_seeds.iter().map(String::as_str).collect()
    };

    let mut everything = Vec::new();

    for seed in seeds {
        let addresses = resolve(seed);
        everything.extend(addresses.iter().copied());

        println!("{SEPARATOR}");
        if addresses.is_empty() {
            println!("Semilla {seed}: no respondió\n");
            continue;
        }

        println!("Semilla {seed}: {} direcciones", addresses.len());
        for (ip, kind) in addresses {
            println!("  {kind:<4}  {ip}");
        }
        println!();
    }

    println!("{SEPARATOR}");
    println!("Total: {} direcciones", everything.len());

    let uniques: HashSet<IpAddr> = everything.into_iter().map(|(ip, _)| ip).collect();
    println!("IPs únicas: {}", uniques.len());
    println!(
        "Un nodo recién arrancado intentaría conectarse por TCP a esas {} direcciones.",
        uniques.len()
    );
}
