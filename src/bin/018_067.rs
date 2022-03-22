fn main() {
    let file = std::path::PathBuf::from(std::env::args().nth(1).expect("no file given"));
    println!("{}", file.as_path().to_string_lossy());

    let data = std::fs::read_to_string(file).expect("unable to read file");
    println!("{}", data);

    let mut triangle: Vec<Vec<u64>> = vec![];
    for line in data.lines() {
        let mut row: Vec<u64> = vec![];
        for number in line.split_whitespace() {
            row.push(number.parse::<u64>().unwrap());
        }
        triangle.push(row);
    }
    println!("{:?}",triangle);

    //Loop up from the bottom of the triangle
    //Keep track of vector of values, 
    //Representing the largest sum from the previous row to the end
    println!();
    let mut largest_sum = triangle.last().cloned().unwrap();
    for row in triangle[0..triangle.len()-1].iter().rev() {
	    println!("{:?}",largest_sum);
	    let mut new_sum = vec![];
	    for n in 0..row.len() {
		    if largest_sum[n] > largest_sum[n+1]{
			    new_sum.push(row[n]+largest_sum[n]);
		    } else {
			    new_sum.push(row[n]+largest_sum[n+1]);
		    }
	    }
	    largest_sum = new_sum;
    }
    println!("{:?}",largest_sum);
}
