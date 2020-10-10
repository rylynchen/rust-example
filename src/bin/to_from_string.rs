use std::fmt;

use serde::export::Formatter;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 8 };
    println!("{}", circle);

    let parsed: i32 = "5".parse().unwrap();
    println!("parsed: {}", parsed);
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("sum is {}", sum);
}