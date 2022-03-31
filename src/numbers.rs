use crate::primes;

use gray_codes::VecSubsets;
use num_bigint::BigUint;
use num_traits::One;
use num_traits::Zero;

pub struct Numbers {
    primes: primes::Primes,
}

impl Numbers {
    pub fn new(primes: primes::Primes) -> Numbers {
        Numbers { primes }
    }

    pub fn prime_factors(&mut self, t: &BigUint) -> Vec<BigUint> {
        let mut prime_factors = vec![];
        let mut max = t.clone();
        for n in self.primes.primes_between(&BigUint::from(2_u32), &t) {
            //Break once were bigger than the current iteration
            if &n > &max {
                break;
            }

            while &max % &n == BigUint::zero() {
                prime_factors.push(n.to_owned());
                max /= &n;
            }
        }
        prime_factors
    }

    pub fn divisors(&mut self, t: &BigUint) -> Vec<BigUint> {
        //Get primes of number
        let mut n_primes = self.prime_factors(&t);
        n_primes.push(BigUint::one());

        //Get vectors of all combinations of the found primes
        let prime_combinations_as_vecs: Vec<_> = VecSubsets::of(&n_primes).collect();

        //Convert vectors of prime combinations into their products
        let mut prime_combinations_as_products = vec![t.to_owned()];
        for prime_vec in prime_combinations_as_vecs {
            let mut product = BigUint::one();
            for prime in prime_vec {
                product *= prime;
            }
            prime_combinations_as_products.push(product)
        }
        prime_combinations_as_products.sort_unstable();
        prime_combinations_as_products.dedup();

        prime_combinations_as_products
    }

    pub fn proper_divisors(&mut self, t: &BigUint) -> Vec<BigUint> {
        let mut proper_divisors = self.divisors(t);
        proper_divisors.pop();
        proper_divisors
    }
}
