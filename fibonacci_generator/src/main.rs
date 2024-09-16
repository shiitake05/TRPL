use std::io;

fn main() {
    println!("知りたいフィボナッチ数列の項数を入力してください");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("入力エラー");
    let n: u32 = n.trim().parse().expect("数値を入力してください");
    println!("フィボナッチ数列の{}番目の値は{}です", n, fibonacci(n));
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}