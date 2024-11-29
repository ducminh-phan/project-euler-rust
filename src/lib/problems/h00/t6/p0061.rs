//! Triangle, square, pentagonal, hexagonal, heptagonal, and octagonal numbers
//! are all figurate (polygonal) numbers and are generated by the following
//! formulae:
//!
//! | Number   | Formula            | Examples             |
//! |----------|--------------------|----------------------|
//! |Triangle  |`P(3,n) = n(n+1)/2` |`1, 3, 6, 10, 15,...` |
//! |Square    |`P(4,n) = n^2`      |`1, 4, 9, 16, 25,...` |
//! |Pentagonal|`P(5,n) = n(3n-1)/2`|`1, 5, 12, 22, 35,...`|
//! |Hexagonal |`P(6,n) = n(2n-1)`  |`1, 6, 15, 28, 45,...`|
//! |Heptagonal|`P(7,n) = n(5n-3)/2`|`1, 7, 18, 34, 55,...`|
//! |Octagonal |`P(8,n) = n(3n-2)`  |`1, 8, 21, 40, 65,...`|
//!
//! The ordered set of three `4`-digit numbers: `8128`, `2882`, `8281`, has
//! three interesting properties.
//!
//! 1. The set is cyclic, in that the last two digits of each number is the
//!    first two digits of the next number (including the last number with the
//!    first).
//! 2. Each polygonal type: triangle (`P(3,127)=8128`), square (`P(4,91)=8281`),
//!    and pentagonal (`P(5,44)=2882`), is represented by a different number in
//!    the set.
//! 3. This is the only set of `4`-digit numbers with this property.
//!
//! Find the sum of the only ordered set of six cyclic `4`-digit numbers for
//! which each polygonal type: triangle, square, pentagonal, hexagonal,
//! heptagonal, and octagonal, is represented by a different number in the set.

use std::collections::HashMap;

use itertools::Itertools;

use crate::numbers::{
    is_heptagonal_number, is_hexagonal_number, is_octagonal_number,
    is_pentagonal_number, is_square, is_triangle_number,
};
use crate::utils::parse_env;

const FUNCTIONS: [fn(u64) -> bool; 6] = [
    is_triangle_number,
    is_square,
    is_pentagonal_number,
    is_hexagonal_number,
    is_heptagonal_number,
    is_octagonal_number,
];

pub fn solve() -> crate::Answer {
    let size: usize = parse_env("SIZE", 6);
    let map = build_map(size);

    // Now we need to find a chain
    // a1 -> k1 -> a2 -> k2 -> ... -> a(s) -> k(s) -> a1
    // such that k1...k(s) is a permutation of 0...(s-1)

    let mut chain_a = vec![];
    let mut visited_k = vec![];

    fn backtrack(
        a: &u64,
        size: &usize,
        map: &HashMap<u64, HashMap<usize, u64>>,
        chain_a: &mut Vec<u64>,
        visited_k: &mut Vec<usize>,
    ) -> bool {
        if let Some(m) = map.get(a) {
            if visited_k.len() == *size - 1 {
                // Now chain_a.len() == size, we need to check if the chain
                // can form a cycle
                let k = (0..*size).find(|k| !visited_k.contains(k)).unwrap();

                return if m.get(&k) == Some(&chain_a[0]) {
                    chain_a.push(*a);
                    true
                } else {
                    false
                };
            }

            let mut found = false;
            chain_a.push(*a);

            for (k, a_next) in m.iter().sorted() {
                if visited_k.contains(k) {
                    continue;
                }

                visited_k.push(*k);
                found = backtrack(a_next, size, map, chain_a, visited_k);

                if !found {
                    visited_k.pop();
                } else {
                    return true;
                }
            }

            if !found {
                chain_a.pop();
            }

            found
        } else {
            false
        }
    }

    let r = map
        .keys()
        .sorted()
        .find(|a| backtrack(a, &size, &map, &mut chain_a, &mut visited_k));

    if r.is_some() {
        let numbers = chain_a
            .iter()
            .enumerate()
            .map(|(i, a)| 100 * a + chain_a[(i + 1) % size])
            .collect::<Vec<_>>();
        dbg!(&numbers);

        numbers.iter().sum::<u64>().into()
    } else {
        panic!("No solution found!");
    }
}

fn build_map(size: usize) -> HashMap<u64, HashMap<usize, u64>> {
    let functions = &FUNCTIONS[..size];

    // Map a -> k -> b, where (100 * a + b) is a k-sided polygonal number
    let mut map = HashMap::new();
    for a in 10..100 {
        let mut m = HashMap::new();
        for b in 10..100 {
            let n = 100 * a + b;
            for (i, f) in functions.iter().enumerate() {
                if f(n) {
                    m.insert(i, b);
                }
            }
        }

        map.insert(a, m);
    }

    map
}
