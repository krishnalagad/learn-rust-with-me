#[derive(Debug)]
enum IpAddrType {
    IPV4(u32, u32, u32, u32),
    IPV6(String)
}

/*
    rustc enums.rs -o app && time ./app && rm app
 */
fn main() {
    let localhost_ipv4 = IpAddrType::IPV4(192, 168, 0, 1);
    println!("{:?}", localhost_ipv4);

    let localhost_ipv6 = IpAddrType::IPV6(String::from("1234:4566:9876:9087"));
    println!("{:?}", localhost_ipv6);
}