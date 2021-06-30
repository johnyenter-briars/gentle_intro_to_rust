use std::env;
use std::fs::File;
use std::io::Read;

fn main(){ 
	let first = env::args().nth(1).expect("Please supply a file name");
	
	let mut file = File::open(&first).expect("Can't open that file");

	let mut text = String::new();

	file.read_to_string(&mut text).expect("Can't read the file!");

	println!("file had {} bytes", text.len());


}