//! By replacing the 1st digit of the 2-digit number `*3`, it turns out that six
//! of the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.
//!
//! By replacing the 3rd and 4th digits of `56**3` with the same digit, this
//! 5-digit number is the first example having seven primes among the ten
//! generated numbers, yielding the family: 56003, 56113, 56333, 56443, 56663,
//! 56773, and 56993. Consequently, 56003, being the first member of this
//! family, is the smallest prime with this property.
//!
//! Find the smallest prime which, by replacing part of the number (not
//! necessarily adjacent digits) with the same digit, is part of an eight prime
//! value family.

use std::collections::HashMap;

use itertools::Itertools;

use crate::primes::{PrimeSet, Primes};

pub fn solve() {
    let result = solve_with_n(8);
    dbg!(result);
}

fn solve_with_n(n: usize) -> u64 {
    let mut masks_map = HashMap::<String, Vec<u64>>::new();
    for p in Primes::new().iter() {
        let masks = generate_masks(p);
        for m in masks.into_iter() {
            let ps = masks_map
                .entry(m)
                .and_modify(|v| v.push(p))
                .or_insert(vec![p]);

            if ps.len() == n {
                return ps[0];
            }
        }
    }

    0
}

fn generate_masks(n: u64) -> Vec<String> {
    let s = n.to_string();
    s.chars()
        .unique()
        // Go over each unique digit
        .flat_map(|digit| {
            s.chars()
                .enumerate()
                // Find indices of such digit
                .filter(move |(_, d)| d == &digit)
                .map(|(i, _)| i)
                // Iterate over non-empty subset of indices
                .powerset()
                .filter(|indices| !indices.is_empty())
                // Replace the digit's occurrences by '*'
                .map(|indices| {
                    let mut chars = s.chars().collect::<Vec<_>>();
                    for i in indices {
                        chars[i] = '*';
                    }
                    chars.iter().collect::<String>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_n() {
        assert_eq!(solve_with_n(6), 13);
        assert_eq!(solve_with_n(7), 56003);
    }

    #[test]
    fn generate_masks_test() {
        assert_eq!(
            generate_masks(56663).iter().sorted().collect::<Vec<_>>(),
            vec![
                "*6663", "5***3", "5**63", "5*6*3", "5*663", "56**3", "56*63",
                "566*3", "5666*"
            ]
        )
    }
}
