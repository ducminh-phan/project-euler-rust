//! The series, `1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317`.
//!
//! Find the last ten digits of the series `1^1 + 2^2 + 3^3 + ... + 1000^1000`.

use num::BigUint;

pub fn solve() {
    let modulus = BigUint::from(10u32).pow(10);
    let r = (1..=1000u32)
        .map(|n| {
            let n = BigUint::from(n);
            n.modpow(&n, &modulus)
        })
        .sum::<BigUint>()
        % modulus;

    println!("{}", r);
}
