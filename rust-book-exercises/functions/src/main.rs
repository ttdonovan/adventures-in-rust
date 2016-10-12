#[allow(unused_variables)]
fn main() {
    println!("Hello, world!");

    another_function();

    another_function_with_args(5, 6);

    let a = 5;

    let b = {
        let a = 3;
        a + 1
    };

    println!("The value of b: {}", b);

    let c = plus_one(5);

    println!("The value of c is: {}", c);
}

fn another_function() {
    println!("Another function");
}

fn another_function_with_args(x: i32, y: i32) {
    println!("The value of x: {}", x);
    println!("The value of y: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
