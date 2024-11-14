//! The square root of `2` can be written as an infinite continued fraction.
//!
//! ```text
//! sqrt(2) = 1 + 1/(2 + 1/(2 + 1/(2 + 1/(2 + ...))))
//! ```
//!
//! The infinite continued fraction can be written, `sqrt(2) = [1; (2)]`, `(2)`
//! indicates that `2` repeats *ad infinitum*. In a similar way,
//! `sqrt(23) = [4; (1, 3, 1, 8)]`.
//!
//! It turns out that the sequence of partial values of continued fractions for
//! square roots provide the best rational approximations. Let us consider the
//! convergents for `sqrt(2)`.
//!
//! ```text
//! 1 + 1/2 = 3/2
//! 1 + 1/(2 + 1/2) = 7/5
//! 1 + 1/(1 + 1/(2 + 1/2)) = 17/12
//! 1 + 1/(1 + 1/(1 + 1/(2 + 1/2))) = 41/29
//! ```
//!
//! Hence the sequence of the first ten convergents for `sqrt(2)` are:
//!
//! ```text
//! 1, 3/2, 7/5, 17/12, 41/29, 99/70, 239/169, 577/408, 1393/985, 3363/2378,...
//! ```
//!
//! What is most surprising is that the important mathematical constant,  
//! ```text
//! e = [2; 1, 2, 1, 1, 4, 1, 1, 6, 1, ... , 1, 2k, 1, ...]
//! ```
//!
//! The first ten terms in the sequence of convergents for `e` are:
//!
//! ```text
//! 2, 3, 8/3, 11/4, 19/7, 87/32, 106/39, 193/71, 1264/465, 1457/536,...
//! ```
//!
//! The sum of digits in the numerator of the `10`th convergent is
//! `1 + 4 + 5 + 7 = 17`.
//!
//! Find the sum of digits in the numerator of the `100`th convergent of the
//! continued fraction for `e`.

use itertools::Itertools;
use num::integer::gcd;
use num::{BigUint, One};

use crate::numbers::sum_of_digits;

pub fn solve() {
    // The 100th convergent is [2; 1, 2, 1,..., 1, 66, 1], we will compute
    // backward, start with x1 = 1, then x2 = 66 + 1/x1, x3 = 1 + 1/x2,...
    let (n, _) = [BigUint::from(2u8)]
        .into_iter()
        .chain((1u8..).flat_map(|k| {
            [BigUint::one(), BigUint::from(2 * k), BigUint::one()]
        }))
        // Take 99 as we will fold the vector starting from 1
        .take(99)
        .collect_vec()
        .into_iter()
        .rev()
        .fold((BigUint::one(), BigUint::one()), |(n, d), k| {
            // Compute k + 1/x = k + d/n = (d + n*k)/n
            let n_new = &d + &n * k;
            let d_new = n.clone();
            let g = gcd(n_new.clone(), d_new.clone());

            (n_new / &g, d_new / &g)
        });

    let result = sum_of_digits(n);
    println!("{result}");
}
