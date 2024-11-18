//! Consider the fraction, `n/d`, where `n` and `d` are positive integers. If
//! `n < d` and `gcd(n,d) = 1`, it is called a reduced proper fraction.
//!
//! If we list the set of reduced proper fractions for `d <= 8` in ascending
//! order of size, we get:
//!
//! ```text
//! 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8
//! ```
//!
//! It can be seen that there are `21` elements in this set.
//!
//! How many elements would be contained in the set of reduced proper fractions
//! for `d < 1,000,000`?

use crate::numbers::compute_phi_to_n;
use crate::utils::parse_env;

pub fn solve() {
    let ceiling = parse_env("CEILING", 1e6 as u64);
    let phi = compute_phi_to_n(ceiling);
    let result = phi[2..=ceiling as usize].iter().sum::<u64>();
    println!("{result}");
}
