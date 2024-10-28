//! 2520 is the smallest number that can be divided by each of the numbers
//! from 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is **evenly divisible** by all of the numbers from 1 to 20
//!

use num::Integer;

pub fn main() {
    let result: u32 = (1..20).fold(1, |p, n| p.lcm(&n));

    println!("{}", result);
}
