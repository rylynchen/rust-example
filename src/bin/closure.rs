fn main() {
    let haystack = vec![1, 2, 3];
    let contains = |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", haystack.len());
}