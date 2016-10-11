use std::io;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m; m = n; n = t;
        }
        m = m % n;
    }
  n
}

fn main() {
    println!("Find the greatest common divisor (gcd) of two numbers (n, m):");

    loop {
        println!("Input for n:");

        let mut n = String::new();

        io::stdin().read_line(&mut n)
            .expect("Failed to read line");

        let n : u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue
        };

        println!("Input for m:");

        let mut m = String::new();

        io::stdin().read_line(&mut m)
            .expect("Failed to read line");

        let m : u64 = match m.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue
        };

        println!("gcd({:?}, {:?}) -> {:?}", n, m, gcd(n, m));
    }
}
