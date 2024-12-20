//! The n-th term of the sequence of triangle numbers is given by
//!
//! ```text
//! t_n = n * (n + 1) / 2
//! ```
//!
//! so the first ten triangle numbers are:
//!
//! ```text
//! 1, 3, 6, 10, 15, 21, 28, 36, 45, 55,...
//! ```
//!
//! By converting each letter in a word to a number corresponding to its
//! alphabetical position and adding these values we form a word value. For
//! example, the word value for SKY is `19 + 11 + 25 = 55 = t_10`. If the word
//! value is a triangle number then we shall call the word a triangle word.
//!
//! Using assets/0042_words.txt, a 16K text file containing nearly two-thousand
//! common English words, how many are triangle words?

use crate::numbers::is_triangle_number;
use crate::utils::{read_file, word_score};

pub fn solve() -> crate::Answer {
    read_file("assets/0042_words.txt", ',')
        .iter()
        .map(word_score)
        .filter(|s| is_triangle_number(*s as u64))
        .count()
        .into()
}
