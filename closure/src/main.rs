fn main() {
    let x = vec![1, 2, 3];
    let equal_to_x = |z| z == x;
    let y = vec![1, 2, 3];
    println!("{}", equal_to_x(y));
    println!("{:?}",x);
}
