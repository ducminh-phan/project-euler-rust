//! Using assets/0022_names.txt, a 46K text file containing over five-thousand
//! first names, begin by sorting it into alphabetical order. Then working out
//! the alphabetical value for each name, multiply this value by its
//! alphabetical position in the list to obtain a name score.
//!
//! For example, when the list is sorted into alphabetical order, COLIN,
//! which is worth `3 + 15 + 12 + 9 + 14 = 53`, is the 938th name in the list.
//! So, COLIN would obtain a score of `938 * 53 = 49714`.
//!
//! What is the total of all the name scores in the file?

use itertools::Itertools;

use crate::utils::{read_file, word_score};

pub fn solve() {
    let result: u32 = read_file("assets/0022_names.txt")
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, s)| (i as u32 + 1u32) * word_score(s))
        .sum();

    println!("{}", result);
}
