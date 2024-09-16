// フィールド
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// struct User {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

// インスタンス
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    println!("{}", user1.email);



    // let user2 = User {
        // email: String::from("another@example.com"),
        // username: String::from("anotherusername567"),
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
    // };

    // 上と同様
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("{}", user2.username);


    // タプル構造体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 20);
    let origin = Point(0, 10, 0);

    println!("{}", black.2);
    println!("{}", origin.1);


    // let user1 = User {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // };

}

// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// 上と同様
// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }