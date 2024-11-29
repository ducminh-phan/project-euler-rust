//! By replacing each of the letters in the word CARE with `1`, `2`, `9`, and
//! `6` respectively, we form a square number: `1296 = 36^2`. What is remarkable
//! is that, by using the same digital substitutions, the anagram, RACE, also
//! forms a square number: `9216 = 96^2`. We shall call CARE (and RACE) a square
//! anagram word pair and specify further that leading zeroes are not permitted,
//! neither may a different letter have the same digital value as another
//! letter.
//!
//! Using `assets/0098_words.txt`, a 16K text file containing nearly
//! two-thousand common English words, find all the square anagram word pairs
//! (a palindromic word is NOT considered to be an anagram of itself).
//!
//! What is the largest square number formed by any member of such a pair?
//!
//! NOTE: All anagrams formed must be contained in the given text file.

use std::collections::HashMap;
use std::iter::zip;

use itertools::Itertools;
use num::integer::Roots;

use crate::numbers::{digits, is_square, num_from_digits};
use crate::utils::read_file;

pub fn solve() -> crate::Answer {
    let words = read_file("assets/0098_words.txt", ',');
    let mut anagram_words = HashMap::new();

    for word in words {
        let sig = chars_signature(&word);
        anagram_words.entry(sig).or_insert(Vec::new()).push(word);
    }

    let to_remove = anagram_words
        .iter()
        .filter(|(_, v)| v.len() == 1)
        .map(|(k, _)| k.to_owned())
        .collect::<Vec<_>>();

    to_remove.iter().for_each(|k| {
        anagram_words.remove(k);
    });

    anagram_words
        .values()
        .flat_map(|v| v.iter().combinations(2))
        .map(|p| (p[0].clone(), p[1].clone()))
        .flat_map(process_anagram_pair)
        .max()
        .unwrap()
        .into()
}

fn chars_signature<S: AsRef<str>>(s: S) -> [u8; 26] {
    let mut sig = [0; 26];
    s.as_ref()
        .as_bytes()
        .iter()
        .for_each(|c| sig[(c - 65) as usize] += 1);

    sig
}

fn process_anagram_pair((first, second): (String, String)) -> Option<u64> {
    let first_chars = first.chars();
    let second_chars = second.chars();

    let n_digits = first.len();
    let min_n = 10u64.pow(n_digits as u32 - 1);
    let max_n = 10u64.pow(n_digits as u32);
    let lower_bound = min_n.sqrt() + 1;
    let upper_bound = max_n.sqrt() - 1;

    let mut max_square: Option<u64> = None;
    for d in lower_bound..=upper_bound {
        let n1 = d * d;
        let chars_map = zip(first_chars.clone(), digits(n1).into_iter().rev())
            .collect::<HashMap<_, _>>();

        if chars_map.values().unique().count() != chars_map.len() {
            continue;
        }

        let n2 = num_from_digits(
            second_chars.clone().map(|c| chars_map[&c]).collect_vec(),
        );

        if n2 >= min_n && n2 <= max_n && is_square(n2) {
            let m = n1.max(n2);

            if let Some(current_max) = max_square {
                max_square = Some(current_max.max(m))
            } else {
                max_square = Some(m);
            }
        }
    }

    max_square
}
