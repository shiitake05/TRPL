#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    // Quarter,
    Quarter(UsState),
}

// ifでは論理値、matchではパターン
// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");   // {}で色々できる
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// エラー（Noneがないため）
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }


fn main() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("{}", value);

    // Nickeとなっているためエラー
    // let coin = Coin::Nicke;
    // let value = value_in_cents(coin);
    // println!("{}", value);

    let coin = Coin::Dime;
    let value = value_in_cents(coin);
    println!("{}", value);

    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin);
    println!("{}", value);


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);      // Some(6)
    println!("{:?}", none);     // None


    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    println!("{:?}", some_u8_value);   // 0

    let some_u8_value = 5u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    println!("{:?}", some_u8_value);   // 5
}