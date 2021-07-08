use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_all_lines(filename: &str) -> io::Result<()> {
	let file = File::open(&filename)?;

	// let reader = io::BufRead::new(file);
	// for line in reader.lines() {
	// 	//the line returned by the iterator is of type io::Result<String> - need to unwrap with ?
	// 	let line = line?;
	// 	println!("line: {}", line);
	// }
	//above is not most efficent - new string is allocated per line

	//better way
	let mut reader = io::BufReader::new(file);
	let mut buf = String::new();
	while reader.read_line(&mut buf)? > 0 {
		{
			let line = buf.trim_right();
			println!("line: {}", line);
		}// need the scope - possible compiler will stop - cause you could use line after you called clear
		buf.clear();
	}
	Ok(())
}

fn main() {
	read_all_lines("file5.rs");
}