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

use std::fs::OpenOptions;
use std::io::Read;

use itertools::Itertools;

pub fn solve() {
    let file = OpenOptions::new()
        .read(true)
        .open("assets/0022_names.txt")
        .unwrap();

    let mut content = String::new();
    std::io::BufReader::new(file)
        .read_to_string(&mut content)
        .unwrap();

    let result: u32 = content
        .split(',')
        .map(|s| s.replace('"', ""))
        .sorted()
        .enumerate()
        .map(|(i, s)| (i as u32 + 1u32) * score(&s))
        .sum();

    println!("{}", result);
}

fn score(s: &String) -> u32 {
    s.as_bytes().iter().map(|c| (c - 65 + 1) as u32).sum()
}
