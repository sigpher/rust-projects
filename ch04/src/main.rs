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

  
}

fn take_ownership(str: String) {
    println!("{}", str);
}