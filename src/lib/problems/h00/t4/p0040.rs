//! An irrational decimal fraction is created by concatenating the positive
//! integers: `0.123456789101112131415161718192021...`
//!
//! It can be seen that the 12th digit of the fractional part is 1.
//!
//! If `d_n` represents the `n`-th digit of the fractional part, find the value
//! of the following expression.
//!
//! ```text
//! d_1 * d_10 * d_100 * d_1000 * d_10000 * d_100000 * d_1000000
//! ```

use crate::numbers::n_digits;

struct Digits {
    number: u32,
    index: u32,
}

impl Digits {
    fn new() -> Self {
        Self {
            number: 1,
            index: 0,
        }
    }
}

impl Iterator for Digits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let digit = ((self.number / 10u32.pow(self.index)) % 10) as u8;

        if self.index == 0 {
            self.number += 1;
            self.index = n_digits(self.number as u64) - 1
        } else {
            self.index -= 1;
        }

        Some(digit)
    }
}

pub fn solve() {
    let indices = (0..=6)
        .map(|p| 10u32.pow(p) as usize)
        .collect::<Vec<usize>>();
    let max_index = indices.iter().max().unwrap();

    let result = Digits::new()
        .enumerate()
        .take_while(|(i, _)| i <= max_index)
        .filter(|(i, _)| indices.contains(&(i + 1)))
        .map(|(_, d)| d as u32)
        .product::<u32>();

    println!("{result}");
}
