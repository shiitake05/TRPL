// fn main() {
//     let some_u8_value = Some(0u8);
//     match some_u8_value {
//         Some(3) => println!("three"),
//         _ => println!("not three"),
//     }
//     let some_u8_value = Some(3u8);
//     match some_u8_value {
//         Some(3) => println!("three"),
//         _ => println!("not three"),
//     }
// }

// 上の場合、追加する定型コードが多いため、if let 式を使用する
// しかし、包括性チェックを失ってしまう
// fn main() {
//     let some_u8_value = Some(3u8);
//     if let Some(3) = some_u8_value {
//         println!("three");
//     }

//     let some_u8_value = Some(2u8);
//     if let Some(3) = some_u8_value {
//         println!("three");
//     } else {
//         println!("not three");
//     }
// }

#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // let mut count = 0;
    // let coin = Coin::Quarter(UsState::Alaska);
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    //もしくは

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}