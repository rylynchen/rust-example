fn main() {
    let number = Some(7);
    if let Some(i) = number {
        println!("i is {}", i);
    }
}