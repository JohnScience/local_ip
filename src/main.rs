use std::net::IpAddr;
use local_ip_address::list_afinet_netifas;

fn main() {
    let network_interfaces = list_afinet_netifas().unwrap();
    let my_local_ip = network_interfaces.iter()
        .filter_map(|(_name, ip)| {
            use IpAddr::V4;
            match ip {
                V4(ipv4) if ipv4.octets()[0..3] == [192,168,0] => Some(ipv4),
                _ => None
            }
        })
        .next()
        .unwrap();

    for (name, ip) in network_interfaces.iter() {
        println!("{}:\t{:?}", name, ip);
    }
    println!();
    
    println!("This is my local IP address: {:?}", my_local_ip);
}