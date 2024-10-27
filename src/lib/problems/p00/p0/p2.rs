//! By considering the terms in the Fibonacci sequence whose values do not exceed four million,
//! find the sum of the even-valued terms.

fn main() {
    let mut a = 2u32;
    let mut b = 8u32;
    let mut sum = a;

    // The recursive formula for even Fibonacci numbers
    // is FE(n) = 4 * FE(n - 1) + FE(n - 2)
    while b <= 4e6 as u32 {
        sum += b;
        (a, b) = (b, 4 * b + a);
    }

    println!("{}", sum);
}
