//! The Fibonacci sequence is defined by the recurrence relation:
//! ```text
//! F(n) = F(n-1) + F(n-2), where F(1) = 1 and F(2) = 1
//! ```
//!
//! Hence the first terms will be:
//! ```text
//! F(1) = 1, F(2) = 1, F(3) = 2, F(4) = 3, F(5) = 5, F(6) = 8,...
//! ```
//! The 12th term, `F(12) = 144`, is the first term to contain three digits.
//!
//! What is the index of the first term in the Fibonacci sequence to contain
//! 1000 digits?

pub fn solve() -> crate::Answer {
    // F(n) ≈ phi^n / sqrt(5)
    // F(n) has 1000 digits => F(n) > 10^999 => phi^n > sqrt(5) * 10^999
    // => n > log(sqrt(5), base=phi) / log(phi) + 999 * log(10, base=phi)
    let sqrt5 = 5f64.sqrt();
    let phi = (sqrt5 + 1.0) / 2.0;

    (sqrt5.log(phi) + 999.0 * 10.0f64.log(phi)).ceil().into()
}
