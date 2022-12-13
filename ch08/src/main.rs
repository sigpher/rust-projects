use std::{fmt::Display, hash::Hash};

fn main() {
    //vectors
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is: {}", third);

    match v.get(2) {
        Some(num) => println!("The third element is {}", num),
        None => println!("There is no third element."),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(0.3),
        SpreadsheetCell::Text("blue".into()),
    ];

    for item in row {
        println!("{:?}", item);
    }

    //String
    let mut s = String::new();
    let data = "initial conents";
    let s = data.to_string();
    // let s: String = data.into();

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    println!("{}", s3);

    for c in "中国".chars() {
        println!("{c}")
    }

    for c in "中国".bytes() {
        println!("{c}")
    }

    //HashMap

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let teams = vec![String::from("blue"), String::from("red")];
    let scores = [10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(&field_name, &field_value);
    println!("{:?}", map);

    println!("{field_name}");
    println!("{field_value}");

    println!("{}", map[&field_name]);

    let text = "how do you do";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

#[derive(Debug)]
pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
