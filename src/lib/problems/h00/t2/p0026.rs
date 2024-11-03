//! A unit fraction contains 1 in the numerator. It can be seen that `1/6` has
//! a 1-digit recurring cycle and `1/7` has a 6-digit recurring cycle.
//!
//! Find the value of `d < 1000` for which `1/d` contains the longest recurring
//! cycle in its decimal fraction part.

use std::collections::HashMap;

pub fn solve() {
    let r = (2..1000)
        .map(find_cycle_length)
        .enumerate()
        .max_by_key(|(_, val)| *val)
        .map(|(idx, _)| idx + 2)
        .unwrap();

    println!("{r}");
}

fn find_cycle_length(n: u32) -> u32 {
    // https://www.geeksforgeeks.org/find-length-period-decimal-value-1n/

    let mut map = HashMap::<u32, u32>::new();
    let mut rem = 1;
    let mut i = 1;

    loop {
        rem = (10 * rem) % n;

        if let Some(r) = map.get(&rem) {
            return i - r;
        }

        map.insert(rem, i);
        i += 1;
    }
}
