//! A common security method used for online banking is to ask the user for
//! three random characters from a passcode. For example, if the passcode was
//! 531278, they may ask for the 2nd, 3rd, and 5th characters; the expected
//! reply would be: 317.
//!
//! The text file, `assets/0079_keylog.txt`, contains fifty successful login
//! attempts.
//!
//! Given that the three characters are always asked for in order, analyse the
//! file so as to determine the shortest possible secret passcode of unknown
//! length.

use itertools::Itertools;

use crate::utils::read_file;

pub fn solve() -> crate::Answer {
    let lines = read_file("assets/0079_keylog.txt", '\n')
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect_vec()
        })
        .collect_vec();

    let mut digits = lines.iter().flatten().unique().copied().collect_vec();

    lines
        .iter()
        .flat_map(|line| line.iter().tuple_windows())
        .for_each(|(u, v)| {
            let i = digits.iter().position(|x| x == u).unwrap();
            let j = digits.iter().position(|x| x == v).unwrap();
            if i > j {
                digits.swap(i, j);
            }
        });

    digits.iter().join("").into()
}
