fn main() {
    let mut n_triangle = 1;
    let mut div_count_max = 0;
    println!();
    loop {
        let mut triangle: u64 = 0;
        for t in 1..=n_triangle {
            triangle += t;
        }

        //print!("{}: ", triangle);

        let mut div_count = 0;

        for div in 1..=(triangle / 2 + 1) {
            if triangle % div == 0 {
                div_count += 1;
                //print!("{} ", div);
            }
        }

        //println!("({})", div_count);

        if div_count > div_count_max {
            div_count_max = div_count;
            println!("{}: {}", triangle, div_count);
        }

        n_triangle += 1;

        if div_count > 500 {
            break;
        }
    }
}
