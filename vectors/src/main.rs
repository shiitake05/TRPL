fn main() {
    // ベクタの生成
    // let v: Vec<i32> = Vec::new();   // 空のベクタを作成
    // let v = vec![1, 2, 3];          // これも新しいベクタを生成している

    // ベクタの更新
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // ベクタをドロップすると要素もドロップする
    // {
        // let v = vec![1, 2, 3, 4];

        // vで作業をする
    // }   // <- vはここでスコープを抜け、解放される


    // let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);

    // match v.get(2) {
    //     //                      "3つ目の要素は{}です"
    //     Some(third) => println!("The third element is {}", third),
    //     //               "3つ目の要素はありません。"
    //     None => println!("There is no third element."),
    // }


    // パニック（存在しない要素を参照しているため）
    // let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // println!("The first element is {}", &v[0]);


    // エラー
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);


    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{}", i);
    // }


    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    // }
    // for i in &v {
    //     println!("{}", i);
    // }


    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
        }
    }
}
