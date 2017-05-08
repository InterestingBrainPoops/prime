extern crate prime;

use prime::prime;
use std::time::Instant;

fn main() {
    for v in prime(500) {
        println!("p = {}", v);
    }

    println!("\n\n");
    let max = 100_000_000;
    let now = Instant::now();
    println!("total prime numbers within {}: {}", max, prime(max).count());
    println!("time = {:?}", now.elapsed());
}