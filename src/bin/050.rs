use euler::primes;

fn main() {
    let mut png = primes::Primes::new();
    let primes = png.primes_between_u32(1, 1000000);

    let mut longest_sum = 0;
    let mut longest_sum_elements = 0;
    'P: for p in &primes {
        'N: for n in 0..primes.len() {
            let mut sum = 0;
            let mut sum_primes = vec![];
            for m in n..primes.len() {
                sum += primes[m];
                sum_primes.push(primes[m]);
                if sum == *p {
                    if sum_primes.len() > longest_sum_elements {
                        longest_sum = sum;
                        longest_sum_elements = sum_primes.len();
                    }
                    println!("{} {:?}", p, sum_primes);
                    continue 'P;
                }
                if sum > *p {
                    continue 'N;
                }
            }
        }
    }

    println!("{}", longest_sum);
}
