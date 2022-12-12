use ch06::{plus_one, Coin, UsState};

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::01"));

    println!("{:?}", home);
    println!("{:?}", loopback);
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maxium is configured to be {}", max);
    }
    let coin = Coin::Quarter(UsState::Alaska);

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }

    let opt_a = Some(10);
    if let Some(num) = opt_a {
        println!("{}", num);
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {
    println!("Move {}", num_spaces);
}
