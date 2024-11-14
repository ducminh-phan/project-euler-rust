//! In the United Kingdom the currency is made up of pound (£) and pence (p).
//! There are eight coins in general circulation:
//!
//! ```text
//! 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
//! ```
//!
//! It is possible to make £2 in the following way:
//!
//! ```text
//! 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
//! ```
//!
//! How many different ways can £2 be made using any number of coins?

const N_COINS: usize = 8;
const COINS: [usize; N_COINS] = [1, 2, 5, 10, 20, 50, 100, 200];
const TARGET: usize = 200;

pub fn solve() {
    let mut dp: [[usize; TARGET + 1]; N_COINS + 1] =
        [[0; TARGET + 1]; N_COINS + 1];

    // Represents the base case where the target sum is 0, and there is only
    // one way to make change: by not selecting any coin
    dp[0][0] = 1;

    for i in 1..=N_COINS {
        for j in 0..=TARGET {
            // Add the number of ways to make change without using the current coin
            dp[i][j] += dp[i - 1][j];

            if j >= COINS[i - 1] {
                // Add the number of ways to make change using the current coin
                dp[i][j] += dp[i][j - COINS[i - 1]];
            }
        }
    }

    let result = dp[N_COINS][TARGET];
    println!("{result}");
}
