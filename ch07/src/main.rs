use garden::vegetables::Asparagus;

use crate::sharp::circle::Circle;
use ch07::eat_at_restaurant;

mod garden;
mod sharp;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
    let flower = garden::flowers::Flower {};
    println!("I'm growing {:?}", flower);

    let circle = Circle::new(4.0);
    println!("{}", circle.area());
    eat_at_restaurant();
}
