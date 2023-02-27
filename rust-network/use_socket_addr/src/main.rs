use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

fn main() {
    let mut v4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let mut v6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 0, 1)), 8080);
    assert_eq!(v4.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    v4.set_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    v6.set_port(8000);
    assert_eq!(v6.port(), 8000);
}
