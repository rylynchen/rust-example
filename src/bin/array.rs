use std::mem::size_of_val;

fn main() {
    let xs: [i32;5] = [1,2,3,4,5];
    println!("{}", xs[0]);
    println!("{}", xs.len());
    println!("{}", size_of_val(&xs));
    analyze_slice(&xs);
    analyze_slice(&xs[1 .. 3]);
}

fn analyze_slice(slice: &[i32]) {
    println!("{}", slice[0]);
    println!("{}", slice[1]);
}