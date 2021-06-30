fn main(){ 
	let n = 0;
	let text = match n {
		0 => "zero",
		1 => "one",
		_ => "idk",
	};

	println!("{}", text);

	//notice ranges have three dots and are inclusive
	let text = match n {
		0...3 => "small",
		4...6 => "medium",
		_ => "large"
	};
	
	println!("{}", text);
}