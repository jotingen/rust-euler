use num_bigint::BigUint;
use num_traits::One;

fn main() {
    println!("{} - {} - {}", 10, factorial(10), sum_digits(factorial(10)));
    println!("{} - {} - {}", 100, factorial(100), sum_digits(factorial(100)));
}

fn factorial(n: u64) -> BigUint {
    let mut factorial = BigUint::one();
    for n in 2..=n {
        factorial *= BigUint::from(n);
    }
    factorial
}

fn sum_digits(n: BigUint) -> u64 {
	let mut sum_digits :u64  = 0;
	let n_vec: Vec<char> = n.to_str_radix(10).chars().collect();
	for d in n_vec {
		sum_digits += d.to_digit(10).unwrap() as u64;
	}
	sum_digits
}