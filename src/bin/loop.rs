fn main() {
    'outer : loop {
        'inner : loop {
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", result);
}