#[derive(Debug)]
enum IpAddrType {
    IPV4(u32, u32, u32, u32),
    IPV6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move{x: u32, y: u32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn some_function() {
        println!("Body of some_function!!");
    }
}

/*
    rustc enums.rs -o app && time ./app && rm app
 */
fn main() {
    let localhost_ipv4 = IpAddrType::IPV4(192, 168, 0, 1);
    println!("{:?}", localhost_ipv4);

    let localhost_ipv6 = IpAddrType::IPV6(String::from("1234:4566:9876:9087"));
    println!("{:?}", localhost_ipv6);

    let message = Message::Write(String::from("Black"));
    println!("{:?}", message);
    Message::some_function();
}