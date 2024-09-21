fn main() {
    // ハッシュマップを生成
    use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // println!("{:?}", scores);

    // use std::collections::HashMap;

    // let teams  = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];

    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // println!("{:?}", scores);


    // ハッシュマップと所有権
    // use std::collections::HashMap; 

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // field_nameとfield_valueはこの時点で無効になる。

    // println!("{}, {}", field_name, field_value);    // エラー


    // ハッシュマップから値を取り出す

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);

    // match score {
    //     Some(value) => println!("{}: {}", team_name, value),
    //     None => println!("{}: None", team_name),
    // }


    // forループでハッシュマップのキーと値のペアを走査
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    // ハッシュマップを更新
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);


    // キーに値がなかった時のみ値を挿入する
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);


    // 古い値に基づいて値を更新
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}