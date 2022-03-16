  use num_bigint::BigUint;
  use num_traits::Zero;
  use num_traits::One;


    pub struct Primes {
      primes: Vec<BigUint>
    }

    impl Primes {
      pub fn new() -> Primes {
        Primes { primes: vec![One::one()] }
      }

      fn generate_primes_to(&mut self, n: &BigUint) {
        let mut largest_found: BigUint = self.primes.last().unwrap().clone();
        let mut integer_to_check: BigUint = self.primes.last().unwrap().clone() + BigUint::one();
        while &largest_found < n {
          let mut is_prime = true;
          for p in &self.primes {
            if &integer_to_check % p == Zero::zero() && p > &BigUint::one() {
              is_prime = false;
              break;
            }
          }
          if is_prime {
            largest_found = integer_to_check.to_owned();
            self.primes.push(integer_to_check.to_owned());
          }
          integer_to_check += BigUint::one();
        }
      }
      fn generate_primes_to_u8(&mut self, n: u8) {
        self.generate_primes_to(&BigUint::from(n))
      }
      fn generate_primes_to_u16(&mut self, n: u16) {
        self.generate_primes_to(&BigUint::from(n))
      }
      fn generate_primes_to_u32(&mut self, n: u32) {
        self.generate_primes_to(&BigUint::from(n))
      }
      fn generate_primes_to_u64(&mut self, n: u64) {
        self.generate_primes_to(&BigUint::from(n))
      }
      fn generate_primes_to_u128(&mut self, n: u128) {
        self.generate_primes_to(&BigUint::from(n))
      }

      pub fn is_prime(self, n: BigUint) -> bool {
        true
      }
      pub fn is_prime_u8(self, n: u8) -> bool {
        self.is_prime(BigUint::from(n))
      }
      pub fn is_prime_u16(self, n: u16) -> bool {
        self.is_prime(BigUint::from(n))
      }
      pub fn is_prime_u32(self, n: u32) -> bool {
        self.is_prime(BigUint::from(n))
      }
      pub fn is_prime_u64(self, n: u64) -> bool {
        self.is_prime(BigUint::from(n))
      }
      pub fn is_prime_u128(self, n: u128) -> bool {
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

