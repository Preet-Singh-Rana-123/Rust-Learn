// ENUMS
// Enums are versatile tool used to represent a type that can take on one of several possible
// variants.
#![allow(warnings)]
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let mut home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let mut collage = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let mut home = IpAddress::V4(127, 0, 0, 1);
    let mut collage2 = IpAddress::V6(String::from("::1"));
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Enhanced Enum
enum IpAddress {
    V4(u32, u32, u32, u32),
    V6(String),
}
