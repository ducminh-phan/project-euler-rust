//! The sum of the squares of the first ten natural numbers is 385.
//! The square of the sum of the first ten natural numbers is 3025.
//!
//! Hence the difference between the sum of the squares of the first
//! ten natural numbers and the square of the sum is 3025 - 385 = 2640.
//!
//! Find the difference between the sum of the squares of the first
//! one hundred natural numbers and the square of the sum.

pub fn main() {
    // Use explicit formulae to calculate values
    let n = 100;
    let square_of_sum = n * n * (n + 1) * (n + 1) / 4;
    let sum_of_square = n * (n + 1) * (2 * n + 1) / 6;
    println!("{}", square_of_sum - sum_of_square);
}
