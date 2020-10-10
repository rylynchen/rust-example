fn main() {
    let value = 5;
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    let mut mut_value = 6;
    match mut_value {
        ref mut m => {
            *m +=10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }

    let pair = (2, -2);
    match pair {
        (x,y) if x == y => println!("twins"),
        (x,y) if x +y ==0 => println!("add zero"),
        _ => {},
    }

    match Some(43) {
        Some(n @ 42) => println!("Bingo {}",n),
        Some(n) => println!("n is {}", n),
        _ => ()
    }
}