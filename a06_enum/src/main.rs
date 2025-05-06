enum IpAddrKind {
    V4,
    V6,
}

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: std::option::Option<i32> = None;
}
