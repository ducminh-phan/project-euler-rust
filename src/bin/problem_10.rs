//! The sum of the primes below 10 is `2 + 3 + 5 + 7 = 17`.
//!
//! Find the sum of all the primes below two million.

use lib::primes::primes;

fn main() {
    let result: u64 = primes().take_while(|p| *p < 2_000_000).sum();
    println!("{}", result)
}
