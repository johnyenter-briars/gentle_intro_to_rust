use std::thread;
use std::time;

fn main() {
	thread::spawn(|| println!("hello"));
	thread::spawn(|| println!("dolly"));

	println!("you so fine");

	//wait a little bit
	thread::sleep(time::Duration::from_millis(100));
}