fn main() {
    // タプル
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;     // 500
    let six_point_four = x.1;   // 6.4
    let one = x.2;               // 1

    // 配列
    let a = [3; 5];     // [3, 3, 3, 3, 3]
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}