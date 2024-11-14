//! The primes `3`, `7`, `109`, and `673`, are quite remarkable. By taking any
//! two primes and concatenating them in any order the result will always be
//! prime. For example, taking `7` and `109`, both `7109` and `1097` are prime.
//! The sum of these four primes, `792`, represents the lowest sum for a set of
//! four primes with this property.
//!
//! Find the lowest sum for a set of five primes for which any two primes
//! concatenate to produce another prime.

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::numbers::n_digits;
use crate::primes::{is_prime_fast, PrimeSet, Primes};
use crate::utils::parse_env;

pub fn solve() {
    let ceiling = parse_env("CEILING", 10_000);
    let set_size = parse_env("SET_SIZE", 5);

    let mut neighbors = HashMap::<u64, Vec<u64>>::new();
    let mut primes = Primes::new();
    let ps = primes
        .iter()
        .take_while(|p| *p <= ceiling)
        .collect::<Vec<_>>();

    for p1 in &ps {
        for p2 in ps.iter().take_while(|p2| *p2 < p1) {
            if !is_good_pair(*p1, *p2, &mut primes) {
                continue;
            }

            let p1_neighbors = HashSet::<u64>::from_iter(
                neighbors
                    .entry(*p1)
                    .and_modify(|ns| ns.push(*p2))
                    .or_insert(vec![*p2])
                    .to_owned(),
            );

            let p2_neighbors = HashSet::<u64>::from_iter(
                neighbors.entry(*p2).or_default().to_owned(),
            );

            // Now we need to check that p1_neighbors and p2_neighbors have at
            // least `set_size - 2` elements in common.
            if p1_neighbors.len() < set_size - 1
                || p2_neighbors.len() < set_size - 2
            {
                continue;
            }

            let common =
                p1_neighbors.intersection(&p2_neighbors).collect::<Vec<_>>();

            if common.len() < set_size - 2 {
                continue;
            }

            // We still need to check that all elements in common are neighbours
            // of each other
            if let Some(subset) =
                common.into_iter().sorted().combinations(set_size - 2).find(
                    |subset| {
                        // Iterate over each element `n` of subset
                        subset.iter().enumerate().all(|(i, n)| {
                            // Check if `n` is the neighbor of some `other`
                            subset[..i].iter().all(|other| {
                                neighbors.get(n).map(|x| x.contains(other))
                                    == Some(true)
                            })
                        })
                    },
                )
            {
                dbg!(p1, p2, &subset);

                let result = p1 + p2 + subset.into_iter().sum::<u64>();
                dbg!(result);

                return;
            }
        }
    }

    panic!("No solution found!");
}

fn is_good_pair(p1: u64, p2: u64, primes: &mut Primes) -> bool {
    let n_digits_1 = n_digits(p1);
    let n_digits_2 = n_digits(p2);

    let p12 = p1 * 10u64.pow(n_digits_2) + p2;
    let p21 = p2 * 10u64.pow(n_digits_1) + p1;

    is_prime_fast(p12, primes) && is_prime_fast(p21, primes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_good_pair() {
        let mut primes = Primes::new();

        assert!(is_good_pair(3, 673, &mut primes));
        assert!(is_good_pair(7, 109, &mut primes));
        assert!(is_good_pair(109, 673, &mut primes));
    }
}
