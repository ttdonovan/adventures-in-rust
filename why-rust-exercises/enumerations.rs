use std::io;

fn safe_div(n: i32, d: i32) -> Option<i32> {
    if d == 0 {
        None
    } else {
        Some(n / d)
    }
}

fn main() {
    println!("Safe Divide.");

    loop {
        println!("Numerator:");

        let mut num = String::new();

        io::stdin().read_line(&mut num)
            .expect("Failed to read line");

        let num : i32 = match num.trim().parse() {
            Ok(n)   => n,
            Err(_)  => continue
        };

        println!("Denominator:");

        let mut denom = String::new();

        io::stdin().read_line(&mut denom)
            .expect("Failed to read line");

        let denom : i32 = match denom.trim().parse() {
            Ok(d)   => d,
            Err(_)  => continue
        };

        match safe_div(num, denom) {
            Some(v) => {
                println!("quotient is {}", v);
                continue
            },
            None    => {
                println!("No quotient.");
                break
            }
        }
  }
}
