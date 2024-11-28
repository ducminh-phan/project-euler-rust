//! If a box contains twenty-one coloured discs, composed of fifteen blue discs
//! and six red discs, and two discs were taken at random, it can be seen that
//! the probability of taking two blue discs, `P(BB) = (15/21) * (14/20) = 1/2`.
//!
//! The next such arrangement, for which there is exactly `50%` chance of taking
//! two blue discs at random, is a box containing eighty-five blue discs and
//! thirty-five red discs.
//!
//! By finding the first arrangement to contain over
//! `10^{12} = 1,000,000,000,000` discs in total, determine the number of blue
//! discs that the box would contain.

pub fn solve() {
    // We need to find a, b such that a/b * (a-1)/(b-1) = 1/2
    // <=> u^2 - 2v^2 = -1, where u = 2b - 1, v = 2a - 1
    // The negative Pell equation u^2 - 2v^2 = -1 has the fundamental solution
    // of (1, 1)

    let mut u = 1u64;
    let mut v = 1u64;

    let min_u = 2e12 as u64;

    while u <= min_u {
        (u, v) = (3 * u + 4 * v, 2 * u + 3 * v);
    }

    let result = (v + 1) / 2;
    println!("{result}");
}
