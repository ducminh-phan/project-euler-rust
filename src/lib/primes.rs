pub struct Primes {
    items: Vec<u64>,
    current: u64,
    next: u64,
    trial_1: u64,
    trial_2: u64,
}

pub fn primes() -> Primes {
    Primes {
        items: vec![],
        current: 2,
        next: 3,
        trial_1: 5,
        trial_2: 7,
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let prime = self.current;
        self.current = self.next;
        loop {
            self.next = self.trial_1;
            self.trial_1 = self.trial_2;
            self.trial_2 = self.next + 6;

            let max_p = (self.next as f32).sqrt().ceil() as u64;
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
