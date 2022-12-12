fn main() {
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
