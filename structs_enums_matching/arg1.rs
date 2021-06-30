fn main(){
	for arg in std::env::args() {
		println!("{}", arg);
	}

	let args: Vec<String> = std::env::args().skip(1).collect();
	if args.len() > 0 {
		print!("we have arguments!");
	}

	let first = std::env::args().nth(1).expect("Please supply an argument");

	let n: i32 = first.parse().expect("not an integer!");

	println!("{}", n);
}