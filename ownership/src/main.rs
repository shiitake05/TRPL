fn main() {
    // 変数スコープ
    // {
    //     let s = "hello";   // sは、ここから有効になる

    //     // sで作業をする
    // }                      // このスコープは終わり。もうsは有効ではない

    // String型
    // let s = String::from("hello");
    // 可変化することが可能
    // let mut s = String::from("hello");
    // s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
    // println!("{}", s); // これは`hello, world!`と出力する


    // メモリと確保
    // 変数
    // let x = 5;
    // let y = x;  // xの値をyにコピーする

    // 文字列
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1); // これはエラーになる


    // 変数とデータの相互作用法: クローン
    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);

    // スタックのみのデータ: コピー
    // let x = 5;
    // let y = x;

    // println!("x = {}, y = {}", x, y);


    // 所有権と関数
    // let s = String::from("hello");  // sがスコープに入る

    // takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない

    // let x = 5;                      // xがスコープに入る

    // makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても
                                    // 大丈夫


    // 戻り値とスコープ
    // let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1に
    //                                     // ムーブする

    // let s2 = String::from("hello");     // s2がスコープに入る

    // let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
                                        // 戻り値もs3にムーブされる

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    //'{}'の長さは、{}です
    println!("The length of '{}' is {}.", s2, len);
}

// fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
//     println!("{}", some_string);
// } // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

// fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
//     println!("{}", some_integer);
// } // ここでsome_integerがスコープを抜ける。何も特別なことはない。


// fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
//     // 呼び出した関数にムーブする

//     let some_string = String::from("hello"); // some_stringがスコープに入る

//     some_string                              // some_stringが返され、呼び出し元関数に
//     // ムーブされる
// }

// fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。

//     a_string  // a_stringが返され、呼び出し元関数にムーブされる
// }

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返します

    (s, length)
}