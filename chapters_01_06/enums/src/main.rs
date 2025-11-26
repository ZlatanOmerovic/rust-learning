#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self, by: &str) {
        println!("Calling self by: {}", by);
        dbg!(&self);
    }
}

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("IPv4: {:#?}, IPv6: {:#?}", four, six);

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    dbg!(&home);
    dbg!(&loopback);

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    dbg!(&home);
    dbg!(&loopback);

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    dbg!(&home);
    dbg!(&loopback);

    let message = Message::Write(String::from("hello"));
    dbg!(&message);

    let message_quit = Message::Quit;
    dbg!(&message_quit);

    message_quit.call("let message_quit = Message::Quit;");

    let message_move = Message::Move {x: 1, y: 2};
    dbg!(&message_move);

    let message_change_color = Message::ChangeColor(0, 255, 255);
    dbg!(&message_change_color);

    message_change_color.call("let message_change_color = Message::ChangeColor(0, 255, 255);");

    let some_number = Some(5);
    let some_char = Some('e');

    let mut absent_number: Option<i32> = None;

    dbg!(&some_number);
    dbg!(&some_char);
    dbg!(&absent_number);

    absent_number = Some(5);
    dbg!(&absent_number);

    // let x: i8 = 5;
    let mut y: Option<i8> = Some(5);
    dbg!(&y);
    // y = Some(69*12); // ^^^^^ attempt to compute `69_i8 * 12_i8`, which would overflow
    const A: i8 = (16*7) - 1;
    y = Some(A); // = 111
    dbg!(&y);
}

fn route(ip_kind: IpAddrKind) {
    dbg!(ip_kind);
}
