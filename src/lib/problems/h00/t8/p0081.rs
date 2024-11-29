//! In the `5` by `5` matrix below, the minimal path sum from the top left to
//! the bottom right, by **only moving to the right and down**, is indicated
//! in bold red and is equal to `2427`.
//!
//! ```text
//! *131*   673    234    103    018
//! *201*  *096*  *342*   965    150
//!  630    803   *746*  *422*   111
//!  537    699    497   *121*   956
//!  805    732    524   *037*  *331*
//! ```
//!
//! Find the minimal path sum from the top left to the bottom right by only
//! moving right and down in `assets/0081_matrix.txt`, a 31K text file
//! containing an `80` by `80` matrix.

use itertools::Itertools;

use crate::utils::read_file;

pub fn solve() -> crate::Answer {
    let matrix = read_file("assets/0081_matrix.txt", '\n')
        .iter()
        .map(|line| {
            line.split(',').flat_map(|w| w.parse::<u64>()).collect_vec()
        })
        .collect_vec();
    let n = matrix.len();

    let mut dp = vec![0; n];
    for i in 0..n {
        let mut curr = vec![0; n];
        for j in 0..n {
            if i == 0 && j == 0 {
                curr[j] = matrix[0][0];
            } else {
                let prev = if i == 0 {
                    curr[j - 1]
                } else if j == 0 {
                    dp[j]
                } else {
                    dp[j].min(curr[j - 1])
                };

                curr[j] = matrix[i][j] + prev;
            }
        }

        dp = curr
    }

    dp.last().unwrap().into()
}
