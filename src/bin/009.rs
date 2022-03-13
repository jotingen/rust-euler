fn main() {
    println!("");
    'a: for a in 0..1000 {
        'b: for b in a + 1..1000 {
            'c: for c in b + 1..1000 {
                if a + b + c == 1000 {
                    let ab_squared = a * a + b * b;
                    let c_squared = c * c;
                    print!("\r{:3}^2 + {:3}^2 = {:6} + {:6} = {:7} <> {:7} = {:3}^2 ", a, b, a*a, b*b, ab_squared, c_squared, c);
                    if ab_squared == c_squared {
                        let prod = a * b * c;
                        println!("\n{}", prod);
                        break 'a;
                    }
                }
            }
        }
    }
}
