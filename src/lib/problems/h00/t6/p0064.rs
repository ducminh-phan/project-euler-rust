//! All square roots are periodic when written as continued fractions and can be
//! written in the form:
//!
//! ```text
//! sqrt(N) = a0 + 1 / (a1 + 1/(a2 + 1/(a3 + ...)))
//! ```
//!
//! For example, let us consider `sqrt(23)`:
//!
//! ```text
//! sqrt(23) = 4 + sqrt(23) - 4 = 4 + 1/(1/(sqrt(23) - 4))
//!     = 4 + 1/(1 + (sqrt(23) - 3)/7)
//! ```
//!
//! If we continue we would get the following expansion:
//!
//! ```text
//! sqrt(23) = 4 + 1/(1 + 1/(3 + 1/(1 + 1/(8 + ...))))
//! ```
//!
//! It can be seen that the sequence is repeating. For conciseness, we use the
//! notation `sqrt(23) = [4;(1,3,1,8)]`, to indicate that the block `(1,3,1,8)`
//! repeats indefinitely.
//!
//! The first ten continued fraction representations of (irrational) square roots are:
//!
//! * `sqrt(2) = [1; (2)]`, period=`1`
//! * `sqrt(3) = [1; (1, 2)]`, period=`2`
//! * `sqrt(5) = [2; (4)]`, period=`1`
//! * `sqrt(6) = [2; (2, 4)]`, period=`2`
//! * `sqrt(7) = [2; (1, 1, 1, 4)]`, period=`4`
//! * `sqrt(8) = [2; (1, 4)]`, period=`2`
//! * `sqrt(10) = [3; (6)]`, period=`1`
//! * `sqrt(11) = [3; (3, 6)]`, period=`2`
//! * `sqrt(12) = [3; (2, 6)]`, period=`2`
//! * `sqrt(13) = [3; (1, 1, 1, 1, 6)]`, period=`5`
//!
//! Exactly four continued fractions, for `N <= 13`, have an odd period.
//!
//! How many continued fractions for `N <= 10,000` have an odd period?

use num::integer::{gcd, Roots};
use num::Integer;

use crate::numbers::is_square;

pub fn solve() {
    let result = (2..=10_000)
        .filter(|n| !is_square(*n))
        .map(compute_continued_fractions)
        .filter(|(_, cycle)| cycle.len() % 2 == 1)
        .count();

    dbg!(result);
}

fn compute_continued_fractions(n: i64) -> (i64, Vec<i64>) {
    // Find the nearest square number less than n
    let m = n.sqrt();
    let s = (n as f64).sqrt();

    // Let s = sqrt(n), now we have s = m + 1/x
    // => x = 1/(s - m)
    // => x = (s + m) / (s² - m²)

    // We represent (a * s + b)/d as (a, b, d)
    // => x = (1, m, s² - m²) = (1, m, n - m²)

    let mut cycle = vec![];
    let mut x = (1, m, n - m * m);
    loop {
        let (a, b, d) = x;

        if a == 1 && d == 1 {
            // Now x(n) = s + b for some b
            // => x(n) = (b + m) + 1/x1 and we have a loop
            cycle.push(b + m);
            break;
        }

        let p = (((a as f64) * s + b as f64) / (d as f64)).floor() as i64;
        cycle.push(p);

        // Now x = (a, b, d) = p + (a, b - p*d, d) = p + (a, b', d), we need to
        // compute x_new = 1/(a, b', d) = d/(a * s + b')
        // = d * (a * s - b') / (a² * n - b'²)
        let b1 = p * d - b;
        let d_new = a * a * n - b1 * b1;
        let a_new = a * d;
        let b_new = d * b1;
        let g = gcd3(a_new, b_new, d_new);

        x = (a_new / g, b_new / g, d_new / g);
    }

    (m, cycle)
}

fn gcd3<T: Integer>(a: T, b: T, c: T) -> T {
    gcd(gcd(a, b), c)
}
