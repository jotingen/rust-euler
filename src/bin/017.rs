use core::panic;

fn main() {
    let mut count = 0;
    for n in 1..5 {
        count += count_number(n);
        print!("{} + ", count_number(n))
    }
    count += count_number(5);
    print!("{} = ", count_number(5));
    println!("{}", count);
    assert_eq!(count, 19);

    println!("{}", count_number(342));
    assert_eq!(count_number(342), 23);

    println!("{}", count_number(115));
    assert_eq!(count_number(115), 20);

    count = 0;
    println!("{}", count);
    for n in 1..1000 {
        count += count_number(n);
        print!("{} + ", count_number(n))
    }
    count += "onethousand".chars().count() as u32;
    print!("{} = ", "onethousand".chars().count());
    println!("{}", count);
}

fn count_number(n: u32) -> u32 {
    let ones = n % 10;
    let tens = (n % 100) / 10;
    let hundreds = (n % 1000) / 100;

    let mut count = 0;
    match hundreds {
        0 => count += 0,
        1 => count += "onehundred".chars().count() as u32,
        2 => count += "twohundred".chars().count() as u32,
        3 => count += "threehundred".chars().count() as u32,
        4 => count += "fourhundred".chars().count() as u32,
        5 => count += "fivehundred".chars().count() as u32,
        6 => count += "sixhundred".chars().count() as u32,
        7 => count += "sevenhundred".chars().count() as u32,
        8 => count += "eighthundred".chars().count() as u32,
        9 => count += "ninehundred".chars().count() as u32,
        _ => panic!("Error processing ones"),
    };

    if tens == 0 && ones == 0 {
        return count;
    }

    if hundreds > 0 {
        count += "and".chars().count() as u32
    }

    count + count_tens(n)
}

fn count_tens(n: u32) -> u32 {
    //Teens are different

    let ones = n % 10;
    let tens = (n % 100) / 10;

    match tens {
        0 => count_ones(ones),
        1 => match ones {
            0 => "ten".chars().count() as u32,
            1 => "eleven".chars().count() as u32,
            2 => "twelve".chars().count() as u32,
            3 => "thirteen".chars().count() as u32,
            4 => "fourteen".chars().count() as u32,
            5 => "fifteen".chars().count() as u32,
            6 => "sixteen".chars().count() as u32,
            7 => "seventeen".chars().count() as u32,
            8 => "eighteen".chars().count() as u32,
            9 => "nineteen".chars().count() as u32,
            _ => panic!("Error processing tens"),
        },
        2 => "twenty".chars().count() as u32 + count_ones(ones),
        3 => "thirty".chars().count() as u32 + count_ones(ones),
        4 => "forty".chars().count() as u32 + count_ones(ones),
        5 => "fifty".chars().count() as u32 + count_ones(ones),
        6 => "sixty".chars().count() as u32 + count_ones(ones),
        7 => "seventy".chars().count() as u32 + count_ones(ones),
        8 => "eighty".chars().count() as u32 + count_ones(ones),
        9 => "ninety".chars().count() as u32 + count_ones(ones),
        _ => panic!("Error processing tens"),
    }
}

fn count_ones(n: u32) -> u32 {
    let ones = n % 10;
    match ones {
        0 => 0,
        1 => "one".chars().count() as u32,
        2 => "two".chars().count() as u32,
        3 => "three".chars().count() as u32,
        4 => "four".chars().count() as u32,
        5 => "five".chars().count() as u32,
        6 => "six".chars().count() as u32,
        7 => "seven".chars().count() as u32,
        8 => "eight".chars().count() as u32,
        9 => "nine".chars().count() as u32,
        _ => panic!("Error processing ones"),
    }
}
