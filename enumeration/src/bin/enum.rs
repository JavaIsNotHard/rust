enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V6(String),
}

// we cannot do this using struct
enum IpAddrEnumTwo {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };

    let loopback = IpAddrEnum::V4(String::from("::1"));
    match loopback {
        IpAddrEnum::V4(_) => println!("IPv4 address"),
        IpAddrEnum::V6(_) => println!("IPv6 address"),
    }
}

fn route(ipkind: IpAddrKind) {}
