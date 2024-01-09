//! If the numbers `1` to `5` are written out in words: one, two, three, four, five,
//! then there are `3 + 3 + 5 + 4 + 4 = 19` letters used in total.
//!
//! If all the numbers from `1` to `1000` (one thousand) inclusive were written
//! out in words, how many letters would be used?
//!
//! Do not count spaces or hyphens. For example, `342` (three hundred and forty-two)
//! contains `23` letters and `115` (one hundred and fifteen) contains `20` letters.
//! The use of "and" when writing out numbers is in compliance with British usage.

use std::collections::HashMap;

fn num_to_text(num: u32) -> String {
    let map = HashMap::from([
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
        (1000, "one thousand"),
    ]);

    if map.contains_key(&num) {
        return (*map.get(&num).unwrap()).into();
    }

    if num <= 99 {
        let rem = num % 10;
        let tens = num - rem;

        return format!("{}-{}", map.get(&tens).unwrap(), map.get(&rem).unwrap());
    }

    if num <= 999 {
        let rem = num % 100;
        let huns = num / 100;

        let mut text = format!("{} hundred", map.get(&huns).unwrap());

        if rem > 0 {
            text = format!("{} and {}", text, num_to_text(rem));
        }

        return text;
    }

    unreachable!()
}

fn main() {
    let result: usize = (1..=1000)
        .map(num_to_text)
        .map(|s| s.replace([' ', '-'], ""))
        .map(|s| s.len())
        .sum();

    println!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_to_word() {
        assert_eq!(num_to_text(5), "five");
        assert_eq!(num_to_text(10), "ten");
        assert_eq!(num_to_text(15), "fifteen");
        assert_eq!(num_to_text(25), "twenty-five");
        assert_eq!(num_to_text(110), "one hundred and ten");
        assert_eq!(num_to_text(125), "one hundred and twenty-five");
        assert_eq!(num_to_text(320), "three hundred and twenty");
    }
}
