// fn main() {
    // let a = [1, 2, 3, 4, 5];

    // let months = ["January", "February", "March", "April", "May", "June", "July",
    //           "August", "September", "October", "November", "December"];

    // let a: [i32; 5] = [1, 2, 3, 4, 5];
//     let a = [3; 5]; // [3, 3, 3, 3, 3]

//     println!("The first month is {}", months[0]);
//     println!("The second month is {}", a[1]);
// }


// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     let first = a[0];
//     let second = a[1];

//     println!("The first element is {}", first);
//     println!("The second element is {}", second);
// }


use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
           // 配列の何番目の要素にアクセスするか指定してください

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
              // 値の読み込みに失敗しました

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
        // 入力された値は数字ではありません

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        // {}番目の要素の値は{}です
        index, element
    );
}