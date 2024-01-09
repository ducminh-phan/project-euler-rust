pub trait PrimeSet: Sized {
    /// Finds one more prime, and adds it to the list
    fn expand(&mut self);

    /// Return all primes found so far as a slice
    fn list(&self) -> &[u64];

    /// Iterator over all primes, starting with 2. If you don't care about the "state" of the
    /// `PrimeSet`, this is what you want!
    fn iter(&mut self) -> PrimeSetIterator<Self> {
        PrimeSetIterator { primes: self, n: 0 }
    }
}

pub struct Primes {
    items: Vec<u64>,
    current: u64,
    next: u64,
    trial_1: u64,
    trial_2: u64,
}

#[allow(clippy::new_without_default)]
impl Primes {
    pub fn new() -> Self {
        Self {
            items: vec![],
            current: 2,
            next: 3,
            trial_1: 5,
            trial_2: 7,
        }
    }
}

impl PrimeSet for Primes {
    /// Finds one more prime, and adds it to the list
    fn expand(&mut self) {
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
    }

    /// Return all primes found so far as a slice
    fn list(&self) -> &[u64] {
        &self.items[..]
    }
}

pub struct PrimeSetIterator<'a, P: PrimeSet> {
    primes: &'a mut P,
    n: usize,
}

impl<'a, P: PrimeSet> Iterator for PrimeSetIterator<'a, P> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.n >= self.primes.list().len() {
            self.primes.expand();
        }
        self.n += 1;

        let m = self.primes.list()[self.n - 1];

        Some(m)
    }
}
