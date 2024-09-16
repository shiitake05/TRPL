fn main() {
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;


    // string型のアドレスを持つenum
    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }


    // 下記のようにも書ける
    // struct Ipv4Addr {
    //     // 省略
    // }
    
    // struct Ipv6Addr {
    //     // 省略
    // }
    
    // enum IpAddr {
    //     V4(Ipv4Addr),
    //     V6(Ipv6Addr),
    // }

    
    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }
    
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }


    // struct QuitMessage; // ユニット構造体
    // struct MoveMessage {
    //     x: i32,
    //     y: i32,
    // }
    // struct WriteMessage(String); // タプル構造体
    // struct ChangeColorMessage(i32, i32, i32); // タプル構造体


    // impl Message {
    //     fn call(&self) {
    //         // method body would be defined here
    //         // メソッド本体はここに定義される
    //     }
    // }
    
    // let m = Message::Write(String::from("hello"));
    // m.call();


    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", absent_number);


    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;
}
