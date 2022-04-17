use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct ContinuedFraction {
    a0: u32,
    an: Vec<u32>,
}

fn calculate_continued_fraction(N: u32) -> ContinuedFraction {
    let mut a0 = 0;
    let mut an = vec![];

    for i in 1..N {
        if i * i < N {
            a0 = i;
        } else {
            break;
        }
    }

    let mut d = 1;
    let mut m = 0;
    let mut a = a0;

    while a != 2*a0 {
	    m = d*a -m;
	    d = (N -m*m) /d;
	    a = (a0 + m) /d;
	    an.push(a);
    }

    ContinuedFraction { a0: a0, an: an }
}
fn main() {
    assert_eq!(
        calculate_continued_fraction(2),
        ContinuedFraction { a0: 1, an: vec![2] }
    );
    assert_eq!(
        calculate_continued_fraction(3),
        ContinuedFraction {
            a0: 1,
            an: vec![1, 2]
        }
    );
    assert_eq!(
        calculate_continued_fraction(5),
        ContinuedFraction { a0: 2, an: vec![4] }
    );
    assert_eq!(
        calculate_continued_fraction(6),
        ContinuedFraction {
            a0: 2,
            an: vec![2, 4]
        }
    );
    assert_eq!(
        calculate_continued_fraction(7),
        ContinuedFraction {
            a0: 2,
            an: vec![1, 1, 1, 4]
        }
    );
    assert_eq!(
        calculate_continued_fraction(8),
        ContinuedFraction {
            a0: 2,
            an: vec![1, 4]
        }
    );
    assert_eq!(
        calculate_continued_fraction(10),
        ContinuedFraction { a0: 3, an: vec![6] }
    );
    assert_eq!(
        calculate_continued_fraction(11),
        ContinuedFraction {
            a0: 3,
            an: vec![3, 6]
        }
    );
    assert_eq!(
        calculate_continued_fraction(12),
        ContinuedFraction {
            a0: 3,
            an: vec![2, 6]
        }
    );
    assert_eq!(
        calculate_continued_fraction(13),
        ContinuedFraction {
            a0: 3,
            an: vec![1, 1, 1, 1, 6]
        }
    );

    let mut odd_periods = 0;
    'L: for N in 2..=13 {
	    for i in 2..=N {
		    if i*i == N {
			    continue 'L;
		    }
		    if i*i > N {
			    break;
		    }
	    }
	    let fraction = calculate_continued_fraction(N);
	    println!("sqrt({}) = [{};({})], period={}", N, fraction.a0, fraction.an.iter().format(","), fraction.an.len());
	    if fraction.an.len()%2 == 1 {
		    odd_periods += 1;
	    }
    }
    println!("Odd periods: {}", odd_periods);
    assert_eq!(odd_periods, 4);

    odd_periods = 0;
    'L: for N in 2..=10000 {
	    for i in 2..=N {
		    if i*i == N {
			    continue 'L;
		    }
		    if i*i > N {
			    break;
		    }
	    }
	    let fraction = calculate_continued_fraction(N);
	    println!("sqrt({}) = [{};({})], period={}", N, fraction.a0, fraction.an.iter().format(","), fraction.an.len());
	    if fraction.an.len()%2 == 1 {
		    odd_periods += 1;
	    }
    }
    println!("Odd periods: {}", odd_periods);

}
