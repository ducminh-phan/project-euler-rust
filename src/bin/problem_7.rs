//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
//! we can see that the 6th prime is 13.
//!
//! What is the 10001st prime number?

struct Primes {
    items: Vec<u32>,
    current: u32,
    next: u32,
    trial_1: u32,
    trial_2: u32,
}

impl Primes {
    pub fn new() -> Primes {
        Primes {
            items: vec![],
            current: 2,
            next: 3,
            trial_1: 5,
            trial_2: 7,
        }
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let prime = self.current;
        self.current = self.next;
        loop {
            self.next = self.trial_1;
            self.trial_1 = self.trial_2;
            self.trial_2 = self.next + 6;

            let max_p = (self.next as f32).sqrt().ceil() as u32;
            let is_prime = self
                .items
                .iter()
                .take_while(|p| p <= &&max_p)
                .all(|p| self.next % p != 0);

            if is_prime {
                break;
            }
        }

        self.items.push(prime);
        Some(prime)
    }
}

fn main() {
    let prime = Primes::new().take(10001).last().unwrap();
    println!("{}", prime);
}
