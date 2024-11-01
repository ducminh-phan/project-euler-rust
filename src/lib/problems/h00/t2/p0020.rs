//! `n!` means `n * (n-1) * ... * 3 * 2 * 1`.
//!
//! For example, `10! = 3628800`, and the sum of the digits in the number `10!`
//! is `3 + 6 + 2 + 8 + 8 + 0 + 0 = 27`.
//!
//! Find the sum of the digits in the number `100!`.

use num_bigint::BigUint;

pub fn solve() {
    let mut p = BigUint::from(1u8);
    for n in 2..=100u8 {
        p *= BigUint::from(n);
    }

    let r = p.to_radix_le(10).iter().map(|d| *d as u32).sum::<u32>();

    println!("{r}");
}
