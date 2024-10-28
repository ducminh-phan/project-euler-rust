//! `2**15 = 32768` and the sum of its digits is `3 + 2 + 7 + 6 + 8 = 26.`
//!
//! What is the sum of the digits of the number `2**1000`?

use itertools::Itertools;
use log::debug;

const SIZE: usize = 1000;
type Digits = [u32; SIZE];

pub fn main() {
    let mut digits: Digits = [0; SIZE];
    digits[0] = 1;
    for _ in 0..1000 {
        mul(&mut digits, 2);
        debug_digits(&digits);
    }

    println!("{}", digits.iter().sum::<u32>());
}

fn mul(digits: &mut Digits, n: u32) {
    let mut carry = 0;
    for d in digits.iter_mut() {
        let tmp = n * (*d) + carry;
        *d = tmp % 10;
        carry = tmp / 10;
    }
}

fn debug_digits(digits: &Digits) {
    let ds = digits.iter().rev().skip_while(|d| **d == 0).join("");

    debug!("{}", ds);
}
