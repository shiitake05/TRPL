// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");  // 別の関数
// }


// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {}", x);   // xの値は{}です
// }


// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {}{}", value, unit_label);
// }


// 全体としても文
// fn main() {
//     let y = 6;      // 文
// }

// fn main() {
//     let x = (let y = 6);    // エラー（値を返さない）
// }


// fn main() {
//     let y = {
//         let x = 3;
//         x + 1       // セミコロンをつけると文に変えてしまう
//     };

//     println!("The value of y is: {}", y);       // The value of y is: 4
// }


// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {}", x);   // The value of x is: 5
// }


fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);       // The value of x is: 6 
}

fn plus_one(x: i32) -> i32 {
    x + 1            // セミコロンをつけると文に変えてしまう
}