
fn main() {
	let primes_up_to = 2000000;

	// Seed with 2
	let mut primes = vec![2];
	let mut sum_of_primes: u128 = 2;

	let mut integer_to_check = 3;
	println!("");
	loop {
		let mut is_prime = true;
		for p in &primes {
			if integer_to_check % p == 0 {
				is_prime = false;
				break;
			}
		}
		if is_prime {
			if integer_to_check >= primes_up_to {
				break;
			}
			print!("\r{}",integer_to_check);
			primes.push(integer_to_check);
			sum_of_primes += integer_to_check;
		}
		integer_to_check += 1;
	}

	println!("Primes found:\n\t{:?}", primes);
	println!("Sum of primes: {}", sum_of_primes);
}