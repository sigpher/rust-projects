use ch04::first_word;

fn main() {
    let s1 = String::from("hello world");
    let s2 = &s1[..5];
    println!("{}", s1);
    println!("{}", s2);
    println!("s1 length: {} cap: {}", s1.len(), s1.capacity());

    //no method named `capacity` found for reference `&str` in the current scope
    // println!("s2 length: {} cap: {}", s2.len(), s2.capacity());

    take_ownership(s1);
    // value used here after move
    // take_ownership(s1);

    let text = "hello world";
    let word = first_word(text);
    println!("the first word of `{text}` is {word} ");

    let user1 = User {
        active: true,
        username: "choi".into(),
        email: "caiwy@tdyh.com.cn".into(),
        sign_in_count: 1,
    };

    let user2 = User {
        username: "lora".into(),
        email: "lora@huy.com.cn".into(),
        ..user1
    };

    println!("{:?}", user1);
    println!("{:?}", user2);
    println!("{:?}", User::default());
}

fn take_ownership(str: String) {
    println!("{}", str);
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn new(active: bool, username: String, email: String, sign_in_count: u64) -> Self {
        Self {
            active,
            username,
            email,
            sign_in_count,
        }
    }
    fn default() -> Self {
        User {
            active: true,
            username: "choi".into(),
            email: "caiwy@tdyh.com.cn".into(),
            sign_in_count: 1000,
        }
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
