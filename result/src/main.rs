use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

// ファイルからユーザ名を読み取る関数s
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// ?演算子を使用した場合
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// ?によってさらに簡潔に
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    // let f = File::open("hello.txt");
    // let f: u32 = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         // ファイルを開く際に問題がありました
    //         panic!("There was a problem opening the file: {:?}", error)
    //     },
    // };

    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!(
    //                     //ファイルを作成しようとしましたが、問題がありました
    //                     "Tried to create file but there was a problem: {:?}",
    //                     e
    //                 )
    //             },
    //         }
    //     },
    //     Err(error) => {
    //         panic!(
    //             "There was a problem opening the file: {:?}",
    //             error
    //         )
    //     },
    // };

    // unwrap()
    // let f = File::open("hello.txt").unwrap();

    // expect()
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // let f = File::open("hello.txt").unwrap();


    let f = File::open("hello.txt")?;
}