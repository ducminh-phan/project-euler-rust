//! Each character on a computer is assigned a unique code and the preferred
//! standard is ASCII (American Standard Code for Information Interchange). For
//! example, uppercase A = 65, asterisk (*) = 42, and lowercase k = 107.
//!
//! A modern encryption method is to take a text file, convert the bytes to
//! ASCII, then XOR each byte with a given value, taken from a secret key. The
//! advantage with the XOR function is that using the same encryption key on the
//! cipher text, restores the plain text; for example, `65 XOR 42 = 107`, then
//! `107 XOR 42 = 65`.
//!
//! For unbreakable encryption, the key is the same length as the plain text
//! message, and the key is made up of random bytes. The user would keep the
//! encrypted message and the encryption key in different locations, and without
//! both "halves", it is impossible to decrypt the message.
//!
//! Unfortunately, this method is impractical for most users, so the modified
//! method is to use a password as a key. If the password is shorter than the
//! message, which is likely, the key is repeated cyclically throughout the
//! message. The balance for this method is using a sufficiently long password
//! key for security, but short enough to be memorable.
//!
//! Your task has been made easy, as the encryption key consists of three lower
//! case characters. Using `assets/0059_cipher.txt`, a file containing the
//! encrypted ASCII codes, and the knowledge that the plain text must contain
//! common English words, decrypt the message and find the sum of the ASCII
//! values in the original text.

use std::collections::HashSet;
use std::sync::LazyLock;

use itertools::{iproduct, Itertools};

use crate::utils::read_file;

static COMMON_WORDS: LazyLock<HashSet<Vec<u8>>> = LazyLock::new(|| {
    vec![
        "the", "be", "to", "of", "and", "a", "in", "that", "have", "I", "it",
        "for", "not", "on", "with",
    ]
    .iter()
    .map(|s| s.bytes().collect())
    .collect::<HashSet<_>>()
});

static CIPHER_TEXT: LazyLock<Vec<u8>> = LazyLock::new(|| {
    read_file("assets/0059_cipher.txt", ',')
        .iter()
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<_>>()
});

pub fn solve() {
    let result = iproduct!(0..26, 0..26, 0..26)
        .map(|(a, b, c)| [b'a' + a, b'a' + b, b'a' + c])
        .map(|k| compute_key_score(&k))
        .max_by_key(|(_sum, score)| *score)
        .unwrap()
        .0;

    dbg!(result);
}

fn compute_key_score(key: &[u8; 3]) -> (u64, u64) {
    let (sum, words_count, common_words_count) = CIPHER_TEXT
        .iter()
        .enumerate()
        // Decrypt
        .map(|(i, c)| c ^ key[i % 3])
        // Chunk by spaces
        .chunk_by(|c| *c == b' ')
        .into_iter()
        // Compute counts
        .fold(
            (0u64, 0, 0),
            |(sum, words_count, common_words_count), (is_space, group)| {
                if is_space {
                    return (
                        sum + b' ' as u64,
                        words_count,
                        common_words_count,
                    );
                }

                let word = group.collect::<Vec<_>>();

                return (
                    sum + word.iter().map(|c| *c as u64).sum::<u64>(),
                    words_count + 1usize,
                    common_words_count + COMMON_WORDS.contains(&word) as usize,
                );
            },
        );

    (sum, (100 * common_words_count / words_count) as u64)
}
