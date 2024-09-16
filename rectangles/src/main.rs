// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
        // 長方形の面積は、{}平方ピクセルです
        // "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }


// 構造的となったが、明確性がない
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }


// 構造体を使って明確性を持たせる
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // rect1は{}です
    // println!("rect1 is {}", rect1);      // エラー
    //println!("rect1 is {:?}", rect1);       // これだけだとエラー（#[derive(Debug)]が必要）
    println!("rect1 is {:#?}", rect1);
}