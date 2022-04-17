fn main() {
    let file = std::path::PathBuf::from(std::env::args().nth(1).expect("no file given"));
    println!("{}", file.as_path().to_string_lossy());

    let data = std::fs::read_to_string(file).expect("unable to read file");
    println!("{}", data);

    let mut names = vec![];
    for name in data.split(",") {
        names.push(name.replace("\"", ""));
    }
    for name in &names {
        println!("{}", name);
    }

    names.sort();
    for name in &names {
        println!("{}", name);
    }

    let mut score: u128 = 0;
    for n in 0..names.len() {
	let name = &names[n];
        print!("{} {} ", name, n+1);
	let mut sum = 0;
	for c in name.chars() {
		sum += (c as u8 - 64) as u32;
		print!("{} ", c as u8 - 64);
	}
	sum *= (n+1) as u32;
	score += sum as u128;
	println!(" => {}", sum);
    }
    println!("{}", score);
}
