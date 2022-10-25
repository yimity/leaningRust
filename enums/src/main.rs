#[derive(Debug)]
enum IpType {
    V4,
    V6,
}

/* enum IpAddressType {
    V4(String),
    V6(String),
}

let loopback = IpAddressType::V6(String::from("::1")); */
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!(
            "Hello, Message! {:?}",
            self
        );
    }
}

#[derive(Debug)]
struct IpAddress {
    ip_type: IpType,
    ip_value: String,
}

fn main() {
    let local_v4 = IpAddress {
        ip_type: IpType::V4,
        ip_value: String::from("127.0.0.1"),
    };

    let local_v6 = IpAddress {
        ip_type: IpType::V6,
        ip_value: String::from("::1"),
    };

    let m = Message::Write(String::from("hello"));
    m.call();

    println!(
        "Hello, world! {}, {:#?}",
        local_v4.ip_value, local_v6.ip_type
    );
}
