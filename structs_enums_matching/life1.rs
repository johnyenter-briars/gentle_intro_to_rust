#[derive(Debug)]
struct A {
	s: &'static str //whatever is assigned to this string will live throughout the entire lifetime of the program
}

fn how(i: u32) -> &'static str{ 
	match i {
		0 => "none",
		1 => "one",
		_ => "many"
	}
}

fn main(){ 
	let a = A {s: "hello world"};
	println!("{:?}", a);

	println!("static string slice here: {}", how(5));
}