//! For a number written in Roman numerals to be considered valid there are
//! basic rules which must be followed. Even though the rules allow some numbers
//! to be expressed in more than one way there is always a "best" way of writing
//! a particular number.
//!
//! For example, it would appear that there are at least six ways of writing the
//! number sixteen:
//!
//! ```text
//! IIIIIIIIIIIIIIII  
//! VIIIIIIIIIII  
//! VVIIIIII  
//! XIIIIII  
//! VVVI  
//! XVI
//! ```
//!
//! However, according to the rules only `XIIIIII` and `XVI` are valid, and the last
//! example is considered to be the most efficient, as it uses the least number
//! of numerals.
//!
//! The 11K text file, `assets/0089_roman.txt`, contains one thousand numbers
//! written in valid, but not necessarily minimal, Roman numerals; see
//! [About... Roman Numerals](https://projecteuler.net/about=roman_numerals) for
//! the definitive rules for this problem.
//!
//! Find the number of characters saved by writing each of these in their
//! minimal form.
//!
//! Note: You can assume that all the Roman numerals in the file contain no more
//! than four consecutive identical units.

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;

use itertools::Itertools;

use crate::utils::read_file;

pub fn solve() {
    let result = read_file("assets/0089_roman.txt", '\n')
        .iter()
        .map(|s| s.len() - RomanNumeral::from_str(s).unwrap().to_string().len())
        .sum::<usize>();
    println!("{result}");
}

struct RomanNumeralSystem {
    digits: HashMap<String, u64>,
    subtractive_pairs: HashMap<String, u64>,
    base_values: Vec<(u64, String)>,
}

impl RomanNumeralSystem {
    fn new() -> Self {
        let digits = HashMap::from([
            ("I".to_string(), 1),
            ("V".to_string(), 5),
            ("X".to_string(), 10),
            ("L".to_string(), 50),
            ("C".to_string(), 100),
            ("D".to_string(), 500),
            ("M".to_string(), 1000),
        ]);

        let subtractive_pairs = HashMap::from([
            ("IV".to_string(), 4),
            ("IX".to_string(), 9),
            ("XL".to_string(), 40),
            ("XC".to_string(), 90),
            ("CD".to_string(), 400),
            ("CM".to_string(), 900),
        ]);

        // Map base values to Roman representation and sort decreasing
        let base_values = digits
            .iter()
            .chain(subtractive_pairs.iter())
            .map(|(s, n)| (*n, s.clone()))
            .sorted()
            .rev()
            .collect_vec();

        Self {
            digits,
            subtractive_pairs,
            base_values,
        }
    }
}

static NUMERAL_SYSTEM: LazyLock<RomanNumeralSystem> =
    LazyLock::new(RomanNumeralSystem::new);

#[derive(Debug, PartialEq)]
struct RomanNumeral(u64);

impl FromStr for RomanNumeral {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect_vec();
        let n = chars.len();

        let mut r = 0;
        let mut idx = 0;
        while idx < n {
            if idx < n - 1 {
                // Try to look ahead to see if we have a subtractive pair
                let p = String::from(&s[idx..idx + 2]);
                if let Some(v) = &NUMERAL_SYSTEM.subtractive_pairs.get(&p) {
                    r += *v;
                    idx += 2;
                    continue;
                }
            }

            r += NUMERAL_SYSTEM
                .digits
                .get(&String::from(chars[idx]))
                .unwrap();
            idx += 1;
        }

        Ok(RomanNumeral(r))
    }
}

impl std::fmt::Display for RomanNumeral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut base_index = 0;
        let mut n = self.0;

        while n > 0 {
            let (base, text) = &NUMERAL_SYSTEM.base_values[base_index];
            while n >= *base {
                n -= base;
                write!(f, "{}", text)?;
            }

            base_index += 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_numeral() {
        for (n, s) in [
            (1, "I".to_string()),
            (3, "III".to_string()),
            (4, "IV".to_string()),
            (5, "V".to_string()),
            (8, "VIII".to_string()),
            (9, "IX".to_string()),
            (17, "XVII".to_string()),
            (258, "CCLVIII".to_string()),
            (469, "CDLXIX".to_string()),
            (1993, "MCMXCIII".to_string()),
            (2508, "MMDVIII".to_string()),
            (5269, "MMMMMCCLXIX".to_string()),
        ] {
            assert_eq!(RomanNumeral(n).to_string(), s);
            assert_eq!(RomanNumeral::from_str(&s), Ok(RomanNumeral(n)));
        }
    }
}
