use std::env;
use std::path::PathBuf;

fn main() {
	let home = env::home_dir().expect("no home dir!");

	let mut path = PathBuf::new();

	path.push(home);
	path.push(".cargo");

	if path.is_dir() {
		println!("{}", path.display());
	}

	let mut path = env::current_dir().expect("cant access current dir!");

	loop {
		println!("{}", path.display());
		if !path.pop() {
			break;
		}
	}

	let mut path = env::current_dir().expect("cant access current dir!");

	loop {
		path.push("config.txt");
		if path.is_file() {
			println!("gotcha : {}", path.display());
			break;
		} else {
			path.pop();
		}

		if !path.pop() {
			break;
		}
	}
}