fn apply<F>(mut f:F) where
    F: FnMut() {
    f();
}

fn apply_to_3<F>(f:F) -> i32 where
    F:Fn(i32) -> i32 {
    f(3)
}

fn main() {
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    let diary = || {
        println!("I said {}", greeting);
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        // drop(farewell);
    };
    apply(diary);
    println!("farewell: {}", farewell);
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}