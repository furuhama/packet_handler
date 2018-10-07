use pnet::datalink;
use ipnetwork::IpNetwork::{V4, V6};

pub fn interfaces() {
    println!("index\tname\tprotocol\tip / prefix");
    for interface in datalink::interfaces().into_iter() {
        for ip in interface.ips.into_iter() {
            match ip {
                V4(network) => {
                    println!("{}\t{}\tipv4\t\t{} / {}",
                             interface.index,
                             interface.name,
                             network.ip(),
                             network.prefix());
                },
                V6(network) => {
                    println!("{}\t{}\tipv6\t\t{} / {}",
                             interface.index,
                             interface.name,
                             network.ip(),
                             network.prefix());
                },
            }
        }
    }
}
