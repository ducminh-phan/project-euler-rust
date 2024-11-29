//! Consider the fraction, `n/d`, where `n` and `d` are positive integers. If
//! `n < d` and `gcd(n,d) = 1`, it is called a reduced proper fraction.
//!
//! If we list the set of reduced proper fractions for `d <= 8` in ascending
//! order of size, we get:
//!
//! ```text
//! 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, *2/5*, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8
//! ```
//!
//! It can be seen that `2/5` is the fraction immediately to the left of `3/7`.
//!
//! By listing the set of reduced proper fractions for `d <= 1,000,000` in
//! ascending order of size, find the numerator of the fraction immediately to
//! the left of `3/7`.

use num::integer::gcd;

pub fn solve() -> crate::Answer {
    let ceiling = 1e6 as u64;
    let (n, d) = (8..=ceiling)
        .map(|d| ((3 * d - 1) / 7, d))
        .map(|(n, d)| {
            let g = gcd(n, d);
            (n / g, d / g)
        })
        .max_by(|(n1, d1), (n2, d2)| (n1 * d2).cmp(&(n2 * d1)))
        .unwrap();

    dbg!(n, d);
    n.into()
}
