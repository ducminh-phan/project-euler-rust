//! The cube, `41063625` (`345^3`), can be permuted to produce two other cubes:
//! `56623104` (`384^3`) and `66430125` (`405^3`). In fact, `41063625` is the
//! smallest cube which has exactly three permutations of its digits which are
//! also cube.
//!
//! Find the smallest cube for which exactly five permutations of its digits are
//! cube.

use std::collections::HashMap;

use crate::numbers::digits;

pub fn solve() {
    let mut map = HashMap::<_, Vec<u64>>::new();
    for i in 1u64.. {
        let n = i.pow(3);
        let sig = signature(n);
        let numbers = map.entry(sig).or_default();
        numbers.push(n);

        if numbers.len() == 5 {
            dbg!(&numbers);

            let result = &numbers[0];
            println!("{result}");

            return;
        }
    }
}

fn signature(n: u64) -> [u8; 10] {
    let mut sig = [0; 10];
    for d in digits(n) {
        sig[d as usize] += 1;
    }

    sig
}
