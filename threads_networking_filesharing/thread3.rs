use std::thread;

fn main() {
	let mut threads = Vec::new();

	for i in 0..5 {
		let t = thread::spawn(move || { // note the move keyword - need to move the value INTO the other thread
			println!("hello {}", i);
		});

		threads.push(t);
	}

	for t in threads {
		t.join().expect("thead failed");
	}
}
