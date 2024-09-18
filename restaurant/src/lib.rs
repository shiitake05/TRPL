// mod : モジュール
// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// コンパイルできない（private）
// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
    // 絶対パス
    // crate::front_of_house::hosting::add_to_waitlist();

    // 相対パス
//     front_of_house::hosting::add_to_waitlist();
// }


// mod front_of_house {
//     pub mod hosting {
//         fn add_to_waitlist() {}
//     }
// }

// pubをつけることで外部からアクセス可能
// しかし、エラー（hostingの 中身 は非公開）
// pub fn eat_at_restaurant() {
    // 絶対パス
    // crate::front_of_house::hosting::add_to_waitlist();

    // 相対パス
//     front_of_house::hosting::add_to_waitlist();
// }


// コンパイル可能
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
    // 絶対パス
    // crate::front_of_house::hosting::add_to_waitlist();

    // 相対パス
//     front_of_house::hosting::add_to_waitlist();
// }


// super（別のモジュールに移動しても更新する場所が少なくて済む）
// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }


// 構造体とenumをpubで公開
// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");

//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

    // 下の行のコメントを外すとコンパイルできない
    // meal.seasonal_fruit = String::from("blueberries");
// }


// ヴァリアントを公開
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}