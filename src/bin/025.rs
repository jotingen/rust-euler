use num_bigint::BigUint;

fn main () {

	let mut current = BigUint::from(1_u32);
	let mut previous = BigUint::from(1_u32);
	let mut index = BigUint::from(2_u32);

	while current.to_string().chars().count() < 1000 {

		let tmp = &current + previous;
		previous = current;
		current = tmp;
		index += BigUint::from(1_u32);
		println!("{} {}", index, current.to_string());

	}

}