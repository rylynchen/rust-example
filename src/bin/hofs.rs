fn main() {
    let sum = (0..).map(|x| x * x)
        .take_while(|&x| x < 1000)
        .filter(|&x| x %2 == 1)
        .fold(0, |acc, x| acc + x);
    println!("sum: {}", sum);
}