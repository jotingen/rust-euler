use num_bigint::BigUint;
use num_traits::One;
use num_traits::Zero;

pub struct Primes {
    primes: Vec<BigUint>,
    q: BigUint,
    r: i8,
}

impl Default for Primes {
  fn default() -> Self {
    Primes::new()
  }
}

impl Primes {
    pub fn new() -> Primes {
        Primes {
            primes: vec![
                BigUint::from(1_u32),
                BigUint::from(2_u32),
                BigUint::from(3_u32),
            ],
            q: BigUint::from(1_u32),
            r: -1,
        }
    }

    fn generate_primes_to(&mut self, n: &BigUint) {
        let mut largest_found: BigUint = self.primes.last().unwrap().clone();
        let mut integer_to_check: BigUint;
        'check_loop: while &largest_found < n {
            //Generate next integer to check
            //Basing that all primes will be of the form 6k+-1
            if self.r > 0 {
                self.q += BigUint::one();
                self.r = -1;
                integer_to_check = BigUint::from(6_u32) * &self.q - BigUint::one();
            } else {
                self.r = 1;
                integer_to_check = BigUint::from(6_u32) * &self.q + BigUint::one();
            }
            //Check if any prime divides into number,
            //if so skip to next number
            for p in &self.primes[1..self.primes.len()] {
                //Skip 1
                if &integer_to_check % p == BigUint::zero() {
                    continue 'check_loop;
                }
            }
            largest_found = integer_to_check.to_owned();
            self.primes.push(integer_to_check.to_owned());
        }
    }

    pub fn is_prime(&mut self, n: BigUint) -> bool {
        if &n > self.primes.last().unwrap() {
            self.generate_primes_to(&n);
        }
        self.primes.contains(&n)
    }
    pub fn is_prime_u8(&mut self, n: u8) -> bool {
        self.is_prime(BigUint::from(n))
    }
    pub fn is_prime_u16(&mut self, n: u16) -> bool {
        self.is_prime(BigUint::from(n))
    }
    pub fn is_prime_u32(&mut self, n: u32) -> bool {
        self.is_prime(BigUint::from(n))
    }
    pub fn is_prime_u64(&mut self, n: u64) -> bool {
        self.is_prime(BigUint::from(n))
    }
    pub fn is_prime_u128(&mut self, n: u128) -> bool {
        self.is_prime(BigUint::from(n))
    }

    pub fn primes_between(&mut self, lo: &BigUint, hi: &BigUint) -> Vec<BigUint> {
        let mut primes = vec![];
        if self.primes.last().unwrap() < &hi {
            self.generate_primes_to(hi)
        }
        for n in &self.primes {
            if n < lo {
                continue;
            } else if n > hi {
                break;
            } else {
                primes.push(n.to_owned());
            }
        }
        primes
    }
    pub fn primes_between_u8(&mut self, lo: u8, hi: u8) -> Vec<u8> {
        let mut primes = vec![];
        for n in self.primes_between(&BigUint::from(lo), &BigUint::from(hi)) {
            primes.push(n.to_str_radix(10).parse::<u8>().unwrap());
        }
        primes
    }
    pub fn primes_between_u16(&mut self, lo: u16, hi: u16) -> Vec<u16> {
        let mut primes = vec![];
        for n in self.primes_between(&BigUint::from(lo), &BigUint::from(hi)) {
            primes.push(n.to_str_radix(10).parse::<u16>().unwrap());
        }
        primes
    }
    pub fn primes_between_u32(&mut self, lo: u32, hi: u32) -> Vec<u32> {
        let mut primes = vec![];
        for n in self.primes_between(&BigUint::from(lo), &BigUint::from(hi)) {
            primes.push(n.to_str_radix(10).parse::<u32>().unwrap());
        }
        primes
    }
    pub fn primes_between_u64(&mut self, lo: u64, hi: u64) -> Vec<u64> {
        let mut primes = vec![];
        for n in self.primes_between(&BigUint::from(lo), &BigUint::from(hi)) {
            primes.push(n.to_str_radix(10).parse::<u64>().unwrap());
        }
        primes
    }
    pub fn primes_between_u128(&mut self, lo: u128, hi: u128) -> Vec<u128> {
        let mut primes = vec![];
        for n in self.primes_between(&BigUint::from(lo), &BigUint::from(hi)) {
            primes.push(n.to_str_radix(10).parse::<u128>().unwrap());
        }
        primes
    }
}
