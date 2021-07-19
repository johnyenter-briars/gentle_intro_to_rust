use std::fs::File;
use std::io::prelude::*;
use std::error::Error;


//handle BOTH io errors and conversions error
fn run(file: &str) -> Result<i32, Box<Error>> {
	let mut file = File::open(file)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	Ok(contents.trim().parse()?) //ok to call just parse - since we define i32 up top
}

fn main() {
	let res = run("boxed-error.rs");
	println!("{:?}", res);
}