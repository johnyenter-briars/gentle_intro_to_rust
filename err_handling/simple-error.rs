#[macro_use]
extern crate simple_error;

use std::error::Error;

type BoxResult<T> = Result<T, Box<Error>>;

fn run(s: &str) -> BoxResult<i32> {
	if s.len() == 0 {
		bail!("empty string!");
	}
	Ok(s.trim().parse())
}

fn run2() -> Result<()> {
	use std::env::args;
	use std::fs::File;
	use std::io::BufReader;
	use std::io::prelude::*;

	let file = args().skip(1).next()
				.ok_or(Error::from("provide a file name!"))?;

	//or could use:
	/*
	let file = match args().skip(1).next() {
		Some(s) => s,
		None => bail!("provide a file name!")
	};
	*/
	
	let f = File::open(&file)?;

	let mut l = 0;

	for line in BufReader::new(f).lines() {
		let line = line?;
		println!("line:  {}", line);
		l += 1;
		if l == 10 {
			break;
		}
	}
	
	Ok(())
}

fn main() {
    println!("{:?}", run("23"));
    println!("{:?}", run("2x"));
    println!("{:?}", run(""));
}