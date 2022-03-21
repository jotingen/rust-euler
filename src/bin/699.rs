use euler::primes;

use gray_codes::VecSubsets;

fn main() {
    assert_eq!(T(10_u64.pow(2)), 270);
    assert_eq!(T(10_u64.pow(6)), 26089287);
    println!("{}", T(10_u64.pow(14)));
}

fn T(N: u64) -> u128 {
    let mut prime = primes::Primes::new();
    let mut powers_of_3 = vec![];

    //Must atleast be divisible by 3 to satisfy being a 3^k denominator
    for n in (3..=N).step_by(3) {
        //println!();
        //println!("{}", n);

        //Get all the prime factors of n, excluding 1 and n
        let mut n_prime_factors: Vec<u64> = vec![];
        let mut n_tmp = n;
        for p in prime.primes_between_u64(2, n / 2) {
            //println!("  {} {}", new_n, p);
            while n_tmp % p == 0 {
                n_prime_factors.push(p);
                n_tmp /= p;
            }
        }

        let sum_of_divisors = sum_of_divisors(n, &n_prime_factors);

        let a = sum_of_divisors;
        let b = n as u128;

        //Get prime factors of a and b, excluding 1 and a/b
        let mut a_prime_factors: Vec<u128> = vec![];
        let mut a_tmp = a;
        for p in prime.primes_between_u128(2, a / 2) {
            while a_tmp % p == 0 {
                a_prime_factors.push(p);
                a_tmp /= p;
            }
        }
        let mut b_prime_factors: Vec<u64> = n_prime_factors.clone();

        //println!("  {} {:?}", a, a_prime_factors);
        //println!("  {} {:?}", b, b_prime_factors);

        //Reduce prime factors
        for p in n_prime_factors {
            while a_prime_factors.contains(&(p as u128)) && b_prime_factors.contains(&p) {
                //If they both contain the prime, take it away
                a_prime_factors.swap_remove(
                    a_prime_factors
                        .iter()
                        .position(|x| *x == p.into())
                        .expect("prime not found in a"),
                );
                b_prime_factors.swap_remove(
                    b_prime_factors
                        .iter()
                        .position(|x| *x == p)
                        .expect("prime not found in b"),
                );
            }
        }

        //println!("  {} {:?}", a, a_prime_factors);
        //println!("  {} {:?}", b, b_prime_factors);

        //If b is not 3 and contains any factors other than 3, bail
        if b != 3
            && (b_prime_factors.is_empty()
                || b_prime_factors.len() != b_prime_factors.iter().filter(|&n| *n == 3).count())
        {
            continue;
        }

        //If we are here, add number to powers_of_3
        powers_of_3.push(n as u128);
        let mut a_reduced = 1;
        for prime in a_prime_factors {
            a_reduced *= prime as u128;
        }
        let mut b_reduced = 1;
        for prime in b_prime_factors {
            b_reduced *= prime as u128;
        }
        println!(
            "{:3} {:4} => {}/{} => {}/{}",
            n, sum_of_divisors, a, b, a_reduced, b_reduced,
        );
    }

    powers_of_3.iter().sum()
}

fn sum_of_divisors(n: u64, n_primes: &[u64]) -> u128 {
    if n == 1 {
        return 1;
    }

    if n_primes.is_empty() {
        return n as u128 + 1;
    }
    //println!("{} - {:?}", n, prime_factors);

    //Get vectors of all combinations of the found primes
    let prime_combinations_as_vecs: Vec<_> = VecSubsets::of(n_primes).collect();
    //println!("{} - {:?}", n, prime_combinations_as_vecs);

    //Convert vectors of prime combinations into their products
    let mut prime_combinations_as_products = vec![n as u128];
    for prime_vec in prime_combinations_as_vecs {
        let mut product = 1;
        for prime in prime_vec {
            product *= *prime as u128;
        }
        prime_combinations_as_products.push(product)
    }
    prime_combinations_as_products.sort_unstable();
    prime_combinations_as_products.dedup();

    //let mut divisors = vec![n as u128];
    //for div in 1..=n / 2 {
    //    if n % div == 0 {
    //        divisors.push(div as u128);
    //    }
    //}

    //divisors.sort_unstable();

    //println!("{} - {:?}", n, prime_combinations_as_products);
    //println!("{} - {:?}", n, divisors);
    //println!();

    //assert_eq!(
    //    prime_combinations_as_products.iter().sum::<u128>(),
    //    divisors.iter().sum::<u128>()
    //);

    //Add products and return
    prime_combinations_as_products.iter().sum()
}
