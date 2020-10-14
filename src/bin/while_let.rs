fn main() {
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("i bigger than 9, quit");
            optional = None;
        } else {
            println!("i is {}, continue", i);
            optional = Some(i + 1);
        }
    }
}