//! The arithmetic sequence, 1487, 4817, 8147, in which each of the terms
//! increases by 3330, is unusual in two ways: (i) each of the three terms are
//! prime, and, (ii) each of the 4-digit numbers are permutations of one
//! another.
//!
//! There are no arithmetic sequences made up of three 1-, 2-, or 3-digit
//! primes, exhibiting this property, but there is one other 4-digit increasing
//! sequence.
//!
//! What 12-digit number do you form by concatenating the three terms in this
//! sequence?

use std::collections::HashMap;

use itertools::Itertools;

use crate::numbers::digits;
use crate::primes::{PrimeSet, Primes};

pub fn solve() {
    let mut primes = Primes::new();
    let mut map = HashMap::<String, Vec<u64>>::new();
    for p in primes
        .iter()
        .skip_while(|p| *p < 1000)
        .take_while(|p| *p <= 10000)
    {
        let p_normalized = digits(p).into_iter().sorted().join("");
        map.entry(p_normalized)
            .and_modify(|v| v.push(p))
            .or_insert(vec![p]);
    }

    let map = HashMap::<_, _>::from_iter(map.into_iter().filter(|(p, v)| {
        p != "1478" // Filter out known sequence
        && v.len() >= 3
    }));

    let result = map
        .values()
        .flat_map(|v| find_arithmetic_subsequence(v))
        .map(|v| v.iter().join(""))
        .next()
        .unwrap();
    println!("{result}");
}

fn find_arithmetic_subsequence(nums: &[u64]) -> Option<[u64; 3]> {
    let n = nums.len();
    for i in 0..(n - 2) {
        for j in (i + 1)..(n - 1) {
            let a = nums[i];
            let b = nums[j];
            if 2 * b > a && nums.contains(&(2 * b - a)) {
                return Some([a, b, 2 * b - a]);
            }
        }
    }

    None
}
