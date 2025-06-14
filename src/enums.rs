#[derive(Debug)]

// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }

// enum IpAddrKind {
//     V4,
//     V6,
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// impl Message {
//     fn call(&self) {
//         println!("message received  : {:?}", self);
//     }
// }

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    //        let home = IpAddr {
    //            kind: IpAddrKind::V4,
    //            address: String::from("127.0.0.1"),
    //        };

    //        let loopback = IpAddr {
    //            kind: IpAddrKind::V6,
    //           address: String::from("::1"),
    //    };

    //  let home = IpAddrKind::V4(String::from("127.0.0.1"));
    //  let loopback = IpAddrKind::V6(String::from("::1"));

    // println!("home = {:?}", home);
    // println!("loopback = {:?}", loopback);

    // let m = Message::Write(String::from("hello"));
    // m.call();
    //
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, Rust!"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    msg1.call();
    msg2.call();
    msg3.call();
    msg4.call();

    let some_number = Some(5);
    let some_char = Some('e');
}
