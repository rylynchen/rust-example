enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl Operations {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

enum Number {
    A,
    B,
    C,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    let x = Operations::Add;
    println!("{}", x.run(3, 1));

    println!("zero is {}", Number::A as i32);
    println!("one is {}", Number::B as i32);
    println!("two is {}", Number::C as i32);

    println!("roses are #{:06x}", Color::Red as i32);
}
