//! Let `d(n)` be defined as the sum of proper divisors of `n` (numbers
//! less than `n` which divide evenly into `n`).
//!
//! If `d(a) = b` and `d(b) = a`, where `a != b`, then `a` and `b` are
//! an amicable pair and each of `a` and `b` are called amicable numbers.
//!
//! For example, the proper divisors of `220` are
//! `1, 2, 4, 5, 10, 11, 20, 22, 44, 55` and `110`; therefore `d(220) = 284`.
//! The proper divisors of `284` are `1, 2, 4, 71` and `142`; so `d(284) = 220`.
//!
//! Evaluate the sum of all the amicable numbers under `10000`.

use cached::proc_macro::cached;

#[cached]
fn sum_of_proper_divisors(n: u32) -> u32 {
    (1..=n / 2).filter(|d| n % d == 0).sum()
}

pub fn main() {
    let limit = 10000;
    let mut sum = 0;

    for a in 2..(limit - 1) {
        for b in (a + 1)..limit {
            if sum_of_proper_divisors(a) == b && sum_of_proper_divisors(b) == a
            {
                sum += a + b
            }
        }
    }

    println!("{}", sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_proper_divisors() {
        assert_eq!(sum_of_proper_divisors(2), 1);
        assert_eq!(sum_of_proper_divisors(4), 3);
        assert_eq!(sum_of_proper_divisors(20), 22); // 1, 2, 4, 5, 10
        assert_eq!(sum_of_proper_divisors(25), 6); // 1, 5
        assert_eq!(sum_of_proper_divisors(30), 42); // 1, 2, 3, 5, 6, 10, 15
    }
}
