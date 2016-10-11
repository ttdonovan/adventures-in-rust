fn min<T: Ord>(a: T, b: T) -> T {
    if a <= b { a } else { b }
}

fn main() {
    println!("{:?}", min(10i8, 20));        // 10
    println!("{:?}", min(10, 20u32));       // 10
    println!("{:?}", min("abc", "xyz"));    // abc
    // println!("{:?}", min(10i32, "xyz")); // error: mismatched types.
}
