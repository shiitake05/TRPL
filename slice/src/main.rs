// fn first_word(s: &String) -> usize {
    // let bytes = s.as_bytes();

    // for (i, &item) in bytes.iter().enumerate() {
    //     if item == b' ' {
    //         return i;
    //     }
    // }

    // s.len()
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

fn main() {
    // let mut s = String::from("hello world");

    // let word = first_word(&s); // wordの中身は、値5になる

    // s.clear(); // Stringを空にする。つまり、""と等しくする


    // 文字列スライス
    // let s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    // println!("{} {}", hello, world);

    // let s = String::from("hello");

    // let slice = &s[0..2];
    // println!("{}", slice);
    // let slice = &s[..2];        // 上と等価
    // println!("{}", slice);

    // let s = String::from("hello");

    // let len = s.len();

    // let slice = &s[3..len];
    // println!("{}", slice);
    // let slice = &s[3..];
    // println!("{}", slice);

    // let slice = &s[0..len];
    // println!("{}", slice);
    // let slice = &s[..];
    // println!("{}", slice);


    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear(); // error! （エラー！）
    // println!("the first word is: {}", word);


    // let my_string = String::from("hello world");
    // let word = first_word(&my_string[..]);              // first_wordは`String`のスライスに対して機能する
    // let my_string_literal = "hello world";
    // let word = first_word(&my_string_literal[..]);      // first_wordは文字列リテラルのスライスに対して機能する
    // let word = first_word(my_string_literal);           // 文字列リテラルは、すでに文字列スライスなので、そのまま使うことができる    // 文字列リテラルは「それ自体すでに文字列スライスなので」、
                                                        // スライス記法なしでも機能するのだ！

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    for i in slice {
        println!("{}", i);
    }
}