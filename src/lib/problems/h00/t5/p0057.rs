//! It is possible to show that the square root of two can be expressed as an
//! infinite continued fraction.
//!
//! ```text
//! sqrt(2) = 1 + 1 / (2 + 1 / (1 + ...))
//! ```
//!
//! By expanding this for the first four iterations, we get:
//!
//! * `1 + 1/2 = 3/2 = 1.5`
//! * `1 + 1/(2 + 1/2) = 7/5 = 1.4`
//! * `1 + 1/(2 + 1/(2 + 1/2)) = 17/12 = 1.41666...`
//! * `1 + 1/(2 + 1/(2 + 1/(2 + 1/2))) = 41/29 = 1.41379...`
//!
//! The next three expansions are `99/70`, `239/169`, and `577/408`, but the
//! eighth expansion, `1393/985`, is the first example where the number of
//! digits in the numerator exceeds the number of digits in the denominator.
//!
//! In the first one-thousand expansions, how many fractions contain a numerator
//! with more digits than the denominator?

use num::BigUint;

use crate::numbers::n_digits;

pub fn solve() {
    let (mut n, mut d) = (BigUint::from(3u8), BigUint::from(2u8));
    let mut result = 0;
    for _ in 0..1000 {
        (n, d) = (&n + 2u8 * &d, &n + &d);
        if n_digits(n.clone()) > n_digits(d.clone()) {
            result += 1;
        }
    }

    dbg!(result);
}
