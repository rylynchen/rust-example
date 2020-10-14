fn main() {
    let vec1 = vec![1,2,3];
    println!("Find 2 in vec1: {:?}", vec1.iter().find(|&&x| x == 2));
    let vec2 = vec![4,5,6];
    println!("Find 2 in vec2: {:?}", vec2.into_iter().find(|&x| x == 2));

    let vec = vec![1, 9, 3, 3, 13, 2];
    let a = vec.iter().position(|x| x%2 == 0);
    println!("a: {:?}", a);
    let b = vec.iter().position(|x| x < &0);
    println!("b: {:?}", b);
}