use euler::primes;

fn main() {
    let mut png = primes::Primes::new();
    for a in png.primes_between_u32(1000, 9998) {
        for b in png.primes_between_u32(a + 1, 9999) {
            let c = b + (b - a);
            if c > 9999 {
                break;
            }
            if png.is_prime_u32(c) {
                let mut a_chars = a.to_string().chars().collect::<Vec<_>>();
                let mut b_chars = b.to_string().chars().collect::<Vec<_>>();
                let mut c_chars = c.to_string().chars().collect::<Vec<_>>();

                a_chars.sort_unstable();
                b_chars.sort_unstable();
                c_chars.sort_unstable();

                if a_chars == b_chars && a_chars == c_chars {
                    println!("{} {} {}", a, b, c);
                }
            }
        }
    }
}
