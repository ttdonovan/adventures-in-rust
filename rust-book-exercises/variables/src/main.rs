fn main() {
    // without a `mut` this will get a compile-time error
    //
    // error[E0384]: re-assignment of immutable variable `x`
    // $ rustc --explain E0384
    //
    let mut x = 5; 
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // examples of "Shadowing"
    //
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "     ";
    let spaces = spaces.len();

    println!("The number of spaces: {}", spaces);

    // this will cause a compilte-time error because
    // we are not allowed to mutate a variable's type
    //
    // error[E0308]: mismatched types
    // $ rustc --explain E0308
    //
    // let mut spaces = "     ";
    // spaces = spaces.len();
}
