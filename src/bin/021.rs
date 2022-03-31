use euler::numbers;
use euler::primes;

use gray_codes::VecSubsets;
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use std::collections::HashMap;

fn main() {
    let mut numbers = numbers::Numbers::new(primes::Primes::new());

    assert_eq!(
        d(&BigUint::from(220_u32), &mut numbers),
        BigUint::from(284_u32)
    );
    assert_eq!(
        d(&BigUint::from(284_u32), &mut numbers),
        BigUint::from(220_u32)
    );

    //Build up list of numbers and sum of proper divisors
    let mut sums = HashMap::new();
    for n in 2..10000_u32 {
        sums.insert(n,
            d(&BigUint::from(n), &mut numbers));
    }
    let mut sum = 0;
    for a in 2..10000_u32 {
        //check if sum is within range
        let d_a =sums.get(&a).unwrap().to_u32().unwrap();
        if d_a >= 2 && d_a < 10000 {
            let b = d_a;
            let d_b = sums.get(&b).unwrap().to_u32().unwrap();
            if d_a == b && d_b == a && a != b {
                sum += a;
                println!("a:{} d(a):{}  b:{} d(b):{}", a, d_a, b, d_b);
            }

        }
    }
    println!("{}", sum);
}

fn d(n: &BigUint, numbers: &mut numbers::Numbers) -> BigUint {
    let mut sum = BigUint::from(0_u32);
    for pd in numbers.proper_divisors(n) {
        sum += &pd;
    }
    sum
}
