//TODO See this:
//https://urmaul.com/blog/solution-for-project-euler-problem-700/

use num_bigint::BigUint;

fn main() {
    // m * n mod p
    let m: BigUint = "1504170715041707".parse::<BigUint>().unwrap();
    let p: BigUint = "4503599627370517".parse::<BigUint>().unwrap();
    let mut n: BigUint = "1".parse::<BigUint>().unwrap();

    let mut coins: Vec<BigUint> = vec![];
    println!();
    loop {
        let coin = (&m * &n) % &p;
        print!("\r{}", &n);
        if coins.len() == 0 || coin < *coins.last().unwrap() {
            coins.push(coin.to_owned());
            println!("\r{}*{}%{} = {}", &m, &n, &p, &coin);
        }
        if (coins.len() > 1 && *coins.first().unwrap() == coin)
            || coin == "0".parse::<BigUint>().unwrap()
        {
            break;
        }
        n += "1".parse::<BigUint>().unwrap()
    }

    let mut sum = coins.first().unwrap().to_owned();
    for n in 1..coins.len() {
	    sum += coins[n].to_owned();
    }
    println!("{}", sum);
}
