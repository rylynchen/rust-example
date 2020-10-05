use std::net::UdpSocket;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Unit;

struct Pair(i32, i32);

struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person{name, age};

    println!("{:?}", peter);
}