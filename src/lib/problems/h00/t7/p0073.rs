//! Consider the fraction, `n/d`, where `n` and `d` are positive integers. If
//! `n < d` and `gcd(n,d) = 1`, it is called a reduced proper fraction.
//!
//! If we list the set of reduced proper fractions for `d <= 8` in ascending
//! order of size, we get:
//!
//! ```text
//! 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, *3/8*, *2/5*, *3/7*, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8
//! ```
//!
//! It can be seen that there are `3` fractions between `1/3` and `1/2`.
//!
//! How many fractions lie between `1/3` and `1/2` in the sorted set of reduced
//! proper fractions for `d <= 12,000`?

use num::integer::gcd;

use crate::numbers::compute_phi_to_n;
use crate::utils::parse_env;

pub fn solve() -> crate::Answer {
    let ceiling = parse_env("CEILING", 12000);
    let phi = compute_phi_to_n(ceiling);
    let total_up_to_one_half =
        phi[2..=ceiling as usize].iter().sum::<u64>() / 2;
    let total_up_to_one_third =
        (2..=ceiling).map(count_up_to_one_third).sum::<u64>();

    let result: u64 = total_up_to_one_half - total_up_to_one_third - 1;
    result.into()
}

fn count_up_to_one_third(d: u64) -> u64 {
    let ceiling = (d - 1) / 3;
    (1..=ceiling).filter(|k| gcd(*k, d) == 1).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_up_to_one_third() {
        assert_eq!(count_up_to_one_third(8), 1);
        assert_eq!(count_up_to_one_third(7), 2);
    }
}
