mod primes;

fn main() {

for p in primes::Primes::new().primes_between_u32(900000,1000000) {
	println!("{}",p);
}

}