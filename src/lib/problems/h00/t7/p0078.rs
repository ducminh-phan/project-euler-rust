//! Let `p(n)` represent the number of different ways in which `n` coins can be
//! separated into piles. For example, five coins can be separated into piles
//! in exactly seven different ways, so `p(5)=7`.
//!
//! ```text
//! OOOOO  
//! OOOO O  
//! OOO OO  
//! OOO O O  
//! OO OO O  
//! OO O O O  
//! O O O O O
//! ```
//!
//! Find the least value of `n` for which `p(n)` is divisible by one million.

pub fn solve() -> crate::Answer {
    PartitionNumbers::new(1e6 as i64)
        .enumerate()
        .find(|(_, p)| *p == 0)
        .unwrap()
        .0
        .into()
}

struct PartitionNumbers {
    modulus: i64,
    list: Vec<u64>,
}

impl PartitionNumbers {
    fn new(modulus: i64) -> Self {
        Self {
            modulus,
            list: vec![],
        }
    }
}

impl Iterator for PartitionNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // https://en.wikipedia.org/wiki/Pentagonal_number_theorem#Relation_with_partitions
        // p(n) = sum (-1)^(k-1) p(n - g(k)) over k = 1..n
        // where g(k) is the k-th generalized pentagonal number:
        //       g(k) = k(3k − 1)/2 for k = 1, −1, 2, −2, 3

        let n = self.list.len() as u64;

        let p = if n == 0 {
            1
        } else {
            (1..)
                .map(|k| {
                    let factor = 2 * (k % 2) - 1; // (-1)^(k-1)
                    let terms = [k, -k]
                        .iter()
                        .flat_map(|k| n.checked_sub(g(*k)))
                        .map(|idx| self.list[idx as usize])
                        .collect::<Vec<_>>();

                    (factor, terms)
                })
                .take_while(|(_, terms)| !terms.is_empty())
                .map(|(f, terms)| {
                    f * (terms.into_iter().sum::<u64>() as i64 % self.modulus)
                })
                .fold(0, |acc, x| (acc + x + self.modulus) % self.modulus)
                as u64
        };

        self.list.push(p);
        Some(p)
    }
}

fn g(k: i64) -> u64 {
    (k * (3 * k - 1) / 2) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_numbers() {
        let partition_numbers =
            PartitionNumbers::new(1000).take(10).collect::<Vec<_>>();
        assert_eq!(partition_numbers, vec![1, 1, 2, 3, 5, 7, 11, 15, 22, 30]);

        let partition_numbers =
            PartitionNumbers::new(7).take(10).collect::<Vec<_>>();
        assert_eq!(partition_numbers, vec![1, 1, 2, 3, 5, 0, 4, 1, 1, 2]);
    }
}
