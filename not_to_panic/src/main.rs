use std::net::IpAddr;

// let home: IpAddr = "127.0.0.1".parse().unwrap();
fn main() {
    // loop {
    //     // --snip--
    
    //     let guess: i32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    
    //     if guess < 1 || guess > 100 {
    //         println!("The secret number will be between 1 and 100.");
    //         continue;
    //     }
    
    //     match guess.cmp(&secret_number) {
    //     // --snip--
    //     }
    // }


    pub struct Guess {
        value: u32,
    }
    
    impl Guess {
        pub fn new(value: u32) -> Guess {
            if value < 1 || value > 100 {
                // 予想の値は1から100の範囲でなければなりませんが、{}でした
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
    
            Guess {
                value
            }
        }
    
        pub fn value(&self) -> u32 {
            self.value
        }
    }
}