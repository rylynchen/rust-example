use std::fmt;

use serde::export::Formatter;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Number is {}", self.value)
    }
}

fn main() {
    let num = Number::from(30);
    println!("{}", num);

    let int = 5;
    let n: Number = int.into();
    println!("{}", n);
}
