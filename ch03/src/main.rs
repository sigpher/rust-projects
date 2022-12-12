const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // integers
    let x = 5;
    println!("The value of x is {}", x);
    {
        let x = 6;
        println!("The value of x is {}", x);
    }
    println!("The value of outer x is {}", x);
    println!("{} hours is {} seconds", 3, THREE_HOURS_IN_SECONDS);

    // floating-point numbers
    let a = 2.0;
    let b: f32 = 3.0;
    println!("{}", a + b as f64);

    // chars
    let char = 'z';
    println!("{} {}", char, char as u8);

    println!("a:{}, z:{}", 'a' as u8, 'z' as u8);
    println!("A:{}, Z:{}", 'A' as u8, 'Z' as u8);

    // tubles
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    // arrays

    let arr = [1, 2, 3, 4, 5];
    for item in &arr {
        println!("{}", item);
    }

    let a = [0.2; 10];
    a.iter().for_each(|x| {
        println!("{}", x);
    });

    let arr = [1, 2, 3, 4, 5];
    println!("{}", arr[2]);

    if let Some(3) = arr.get(2) {
        // println!("{}", Some('c').unwrap())
    }

    println!("the value of x is {}", another_function(3));

    let x = {
        let y = 10;
        y + 1
    };

    // flow

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false")
    }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    for number in 1..10 {
        println!("{}", number);
    }
}

fn another_function(x: i32) -> i32 {
    println!("another function");
    x
}
