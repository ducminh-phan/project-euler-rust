//! Comparing two numbers written in index form like `2^11` and `3^7` is not
//! difficult, as any calculator would confirm that `2^11 = 2048 < 3^7 = 2187`.
//!
//! However, confirming that `632382^518061 > 519432^525806` would be much more
//! difficult, as both numbers contain over three million digits.
//!
//! Using `assets/0099_base_exp.txt`, a 22K text file containing one thousand
//! lines with a base/exponent pair on each line, determine which line number
//! has the greatest numerical value.
//!
//! NOTE: The first two lines in the file represent the numbers in the example
//! given above.

use crate::utils::read_file;

pub fn solve() {
    let result = read_file("assets/0099_base_exp.txt", '\n')
        .iter()
        .map(|line| line.split(','))
        .map(|words| {
            words
                .map(|word| word.parse().unwrap())
                .collect::<Vec<f64>>()
        })
        .map(|nums| (nums[0], nums[1]))
        .map(|(b, e)| e * b.log2())
        .enumerate()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .unwrap()
        .0
        + 1;

    println!("{result}");
}
