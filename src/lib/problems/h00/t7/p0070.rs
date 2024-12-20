//! Euler's totient function, `Φ(n)` (sometimes called the phi function), is
//! used to determine the number of positive numbers less than or equal to `n`
//! which are relatively prime to `n`. For example, as `1, 2, 4, 5, 7`, and `8`,
//! are all less than nine and relatively prime to nine, `Φ(9) = 6`.
//!  
//! The number `1` is considered to be relatively prime to every positive
//! number, so `Φ(1) = 1`.
//!
//! Interestingly, `Φ(87109) = 79180`, and it can be seen that `87109` is a
//! permutation of `79180`.
//!
//! Find the value of `n`, `1 < n < 10^7`, for which `Φ(n)` is a permutation of
//! `n` and the ratio `n/Φ(n)` produces a minimum.

use crate::numbers::{compute_phi_to_n, digits};

pub fn solve() -> crate::Answer {
    let ceiling = 1e7 as u64;
    let phi = compute_phi_to_n(ceiling - 1);

    (2..ceiling)
        .map(|n| (n, phi[n as usize]))
        .filter(|(n, f)| signature(*n) == signature(*f))
        .map(|(n, f)| (n, (n as f64) / f as f64))
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .unwrap()
        .0
        .into()
}

fn signature(n: u64) -> [u8; 10] {
    let mut sig = [0; 10];
    for d in digits(n) {
        sig[d as usize] += 1;
    }

    sig
}
