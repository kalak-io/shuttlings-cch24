use std::net::{Ipv4Addr, Ipv6Addr};

pub fn add_ip_addresses(addr1: Ipv4Addr, addr2: Ipv4Addr) -> Ipv4Addr {
    let result: [u8; 4] = addr1
        .octets()
        .iter()
        .zip(addr2.octets().iter())
        .map(|(&a, &b)| (a as u16 + b as u16) as u8)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();

    Ipv4Addr::from(result)
}

pub fn subtract_ip_addresses(addr1: Ipv4Addr, addr2: Ipv4Addr) -> Ipv4Addr {
    let result: [u8; 4] = addr1
        .octets()
        .iter()
        .zip(addr2.octets().iter())
        .map(|(&a, &b)| a.wrapping_sub(b))
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();

    Ipv4Addr::from(result)
}

pub fn xor_ipv6_addresses(addr1: Ipv6Addr, addr2: Ipv6Addr) -> Ipv6Addr {
    let result: [u8; 16] = addr1
        .octets()
        .iter()
        .zip(addr2.octets().iter())
        .map(|(&a, &b)| a ^ b)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();

    Ipv6Addr::from(result)
}
