use std::io;
use std::fs::File;
use std::io::prelude::*;

fn write_out(f: &str) -> io::Result<()>{
	let mut out = File::create(f)?;

	write!(out,"answer is {}\n", 42)?;
	Ok(())
}

fn main() {
	let mut stdout = io::stdout();

	write!(stdout, "answer is {}\n", 42).expect("write failed");

	write_out("test.txt").expect("write failed!");
}