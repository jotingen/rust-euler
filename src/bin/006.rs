fn main() {
    let integers = 100;
    let sum_of_squares = sum_of_squares(integers);
    println!(
        "Sum of squares of first {} integers: {}",
        integers,
        sum_of_squares
    );
    let square_of_sum = square_of_sum(integers);
    println!(
        "Square of sum of first {} integers: {}",
        integers,
        square_of_sum
    );
    let difference = square_of_sum - sum_of_squares;
    println!("Difference: {}",
difference)
}

fn sum_of_squares(integers: u32) -> u64 {
    let mut sum_of_squares = 0;
    for n in 0..=integers as u64 {
        sum_of_squares += n*n;
    }
    sum_of_squares
}

fn square_of_sum(integers: u32) -> u64 {
    let mut sum = 0;
    for n in 0..=integers as u64 {
        sum += n;
    }
    sum*sum
}