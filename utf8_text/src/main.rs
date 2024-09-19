fn main() {
    // let mu?t s = String::new();

    // to_stringメソッドを使ってString型の値を生成する
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", data);
    println!("{}", s);
    let s = "initial contents".to_string();
    println!("{}", s);

    // String::from関数を使ってString型の値を生成する
    let s = String::from("initial contents");
    println!("{}", s);

    // 有効なString型
    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);


    // 文字列の更新
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);


    // 演算子で連結する
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1はムーブされ、もう使用できないことに注意
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);


    // format!マクロを使って文字列を連結する
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);


    // 文字列に添え字アクセスする
    // エラー
    // let s1 = String::from("hello");
    // let h = s1[0];


    // 内部表現
    let len = String::from("Hola").len();
    println!("{}", len);
    let len = String::from("Здравствуйте").len();
    println!("{}", len);    // 24（UTF-8によるエンコード）
    let hello = "Здравствуйте";
    // let answer = &hello[0];      // エラー
    // println!("{}", answer);


    // 文字列をスライスする
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);
    // let s = &hello[0..1];
    // println!("{}", s);  // パニック


    // 文字列を走査するメソッド群
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    
}