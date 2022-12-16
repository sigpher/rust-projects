use std::{
    cmp::Ordering::{Equal, Greater, Less},
    io, process,
    time::SystemTime,
};

use rand::Rng;

pub fn guess_game() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let start = SystemTime::now();
    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Less => println!("Too Small!"),
            Greater => println!("Too Big!"),
            Equal => {
                println!("You Win!!!");
                let end = SystemTime::now();
                let time_use = end.duration_since(start).unwrap();
                println!("使用了{}秒", time_use.as_secs());
                process::exit(0);
                // break;
            }
        }
    }

    // if let Equal = guess.cmp(&secret_number) {
    //     println!("You win!!!")
    // } else {
    //     println!("you lose")
    // }
}
