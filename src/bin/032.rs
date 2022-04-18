use itertools::Itertools;

fn main() {
    let mut products = vec![];
    for a in 1..98765432_u64 {
        for b in a + 1..98765432_u64 {
            let c = a * b;
            let a_length = a.to_string().chars().count();
            let b_length = b.to_string().chars().count();
            let c_length = c.to_string().chars().count();
            if a_length + b_length + c_length > 9 {
                break;
            }
            if a_length + b_length + c_length == 9 {
                let mut characters = vec![];
                characters.append(&mut a.to_string().chars().collect());
                characters.append(&mut b.to_string().chars().collect());
                characters.append(&mut c.to_string().chars().collect());

                if characters.iter().unique().count() == 9 && !characters.contains(&'0') {
                    println!("{} * {} = {}", a, b, c);
                    products.push(c);
                }
            }
        }
    }
    println!("{}", products.iter().unique().sum::<u64>())
}
