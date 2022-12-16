fn main() {
    let closure_plus = |a, b| a * b;

    println!("a + b = {}", calc(add, 10, 30));
    println!("a - b = {}", calc(sub, 10, 30));
    println!("a * b = {}", calc(closure_plus, 10, 30));
  
    println!("------------------------------------");

    let (a, b) = (11, 33);
    println!("a + b = {}", calc2("add")(a, b));

    println!("a * b = {}", calc(closure_plus, 10, 30));
    println!("------------------------------------");
    let res1 = login_required("choi", "123");
    let res2 = login_required("neo", "123");

    for item in [res1, res2] {
        match item {
            Result::Text(error) => println!("{}", error),
            Result::Func(index) => println!("{}", index()),
        }
    }
}
// method as argument
fn calc(method: MethodType, a: i32, b: i32) -> i32 {
    method(a, b)
}

// method as return value
fn calc2(op: &str) -> MethodType {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let sub = |a: i32, b: i32| a - b;

    match op {
        "add" => add,
        "sub" => sub,
        _ => unimplemented!(),
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

type MethodType = fn(i32, i32) -> i32;

enum Result {
    Text(String),
    Func(fn() -> String),
}

fn index() -> String {
    String::from("welcome to my tutorial")
}

fn login_required(username: &str, password: &str) -> Result {
    if !(username == "choi" && password == "123") {
        return Result::Text(String::from("please login first"));
    } else {
        return Result::Func(index);
    }
}
