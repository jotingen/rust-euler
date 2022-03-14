use num_bigint::BigUint;

fn main() {
    let mut number: BigUint = "2".parse::<BigUint>().unwrap();

    number = number.pow(1000);

    println!("{}", number);

    let mut sum = 0;
    for c in number.to_string().chars() {
        let n = c.to_digit(10).unwrap();
        sum += n;
    }

    println!("{}", sum);
}
