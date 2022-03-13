
fn main() {
	let nth_prime_to_find = 10001;

	// Seed with 2
	let mut num_primes = 1;
	let mut primes = vec![2];

	let mut integer_to_check = 3;
	while num_primes < nth_prime_to_find {
		let mut is_prime = true;
		for p in &primes {
			if integer_to_check % p == 0 {
				is_prime = false;
				break;
			}
		}
		if is_prime {
			num_primes += 1;
			primes.push(integer_to_check);
		}
		integer_to_check += 1;
	}

	println!("Primes found:\n\t{:?}", primes);
	println!("Prime #{}: {}", nth_prime_to_find, primes[nth_prime_to_find-1]);
}