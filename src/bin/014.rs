fn main() {
    let mut deepest_number = 0;
    let mut deepest_length = 0;
    for n in 1..1000000 as u128 {
        //print!("{}", n);
        let mut num = n;
	let mut length = 0;
        loop {
            num = next(num);
	    length += 1;
            //print!(" -> {}", num);
            if num == 1 {
		    if length > deepest_length {
			    deepest_number = n;
			    deepest_length = length;
		    }
                //println!();
                break;
            }
        }
    }
    println!("{}: {}", deepest_number, deepest_length);
}

fn next(n: u128) -> u128 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}
