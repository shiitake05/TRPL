// pub mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {
//             println!("Added to waitlist.");
//         }
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     front_of_house::hosting::add_to_waitlist();
//     front_of_house::hosting::add_to_waitlist();
//     front_of_house::hosting::add_to_waitlist();
// }


// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }


// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use self::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }



// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant() {
//     add_to_waitlist();
//     add_to_waitlist();
//     add_to_waitlist();
// }


// 親モジュールを使うことで2つのResult型を区別できる
// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
    // --snip--
    // （略）
// }

// fn function2() -> io::Result<()> {
    // --snip--
    // （略）
// }


// asでリネーム
// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
    // --snip--
// }

// fn function2() -> IoResult<()> {
    // --snip--
// }


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// glob演算子（パスにおいて定義されているすべての公開要素をスコープに持ち込みたいときに使用）
use std::collections::*;