use std::io;

fn main() {
    println!("摂氏から華氏に変換する場合は 'C' を、華氏から摂氏に変換する場合は 'F' を入力してください。");

    let mut conversion_type = String::new();
    io::stdin().read_line(&mut conversion_type).expect("入力エラー");

    if conversion_type.trim().to_uppercase() == "C" {
        println!("摂氏温度を入力してください:");
        let mut celsius = String::new();
        io::stdin().read_line(&mut celsius).expect("入力エラー");

        let celsius: f64 = celsius.trim().parse().expect("数値を入力してください");
        let fahrenheit = celsius_to_fahrenheit(celsius);
        println!("{}度摂氏は{}度華氏です", celsius, fahrenheit);
    } else if conversion_type.trim().to_uppercase() == "F" {
        println!("華氏温度を入力してください:");
        let mut fahrenheit = String::new();
        io::stdin().read_line(&mut fahrenheit).expect("入力エラー");

        let fahrenheit: f64 = fahrenheit.trim().parse().expect("数値を入力してください");
        let celsius = fahrenheit_to_celsius(fahrenheit);
        println!("{}度華氏は{}度摂氏です", fahrenheit, celsius);
    } else {
        println!("無効な入力です。'C' または 'F' を入力してください。");
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}
