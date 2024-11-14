//! 145 is a curious number, as `1! + 4! + 5! = 1 + 24 + 120 = 145`.
//!
//! Find the sum of all numbers which are equal to the sum of the factorial of
//! their digits.
//!
//! Note: As `1! = 1` and `2! = 2` are not sums they are not included.

use crate::numbers::digits;

const DIGIT_FACTORIALS: [u32; 10] =
    [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

pub fn solve() {
    // Assume that a number "n" has "k" digits and equal to the sum of the
    // factorial of its digits => 10^(k-1) < n < k * 9!

    let k = (2..)
        .find(|k| 10u32.pow(k - 1) > (*k) * DIGIT_FACTORIALS[9])
        .unwrap()
        - 1;
    dbg!(k);

    let nums = (3..10u32.pow(k))
        .filter(|n| {
            *n == digits(*n)
                .iter()
                .map(|d| DIGIT_FACTORIALS[*d as usize])
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    dbg!(&nums);

    let result = nums.iter().sum::<u32>();
    println!("{result}");
}
