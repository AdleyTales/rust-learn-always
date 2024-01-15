/// 枚举 enum
/// cargo run --bin 1_枚举_enums

fn main() {
    let ipV4 = IpAddrKind::V4;
    let ipV6 = IpAddrKind::V6;
    println!("ipV4: {:?}", ipV4);
    println!("ipV6: {:?}", ipV6);
}

// IP地址：IPv4、IPv6
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}