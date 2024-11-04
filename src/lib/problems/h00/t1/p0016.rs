//! `2**15 = 32768` and the sum of its digits is `3 + 2 + 7 + 6 + 8 = 26.`
//!
//! What is the sum of the digits of the number `2**1000`?

use num::BigUint;

pub fn solve() {
    let n = num::pow(BigUint::from(2u8), 1000);
    let r = n.to_radix_le(10).iter().map(|d| *d as u32).sum::<u32>();

    println!("{r}");
}
