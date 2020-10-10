fn main() {
    let _unused = 3u32;
    let noisy_unused = 2u32;

    let i = 1;
    let j = 1u8;
    let f = 1.0;
    println!("bytes: {}", std::mem::size_of_val(&i));
    println!("bytes: {}", std::mem::size_of_val(&j));
    println!("bytes: {}", std::mem::size_of_val(&f));

    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);
}
