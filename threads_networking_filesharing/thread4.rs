use std::thread;
use std::sync::Arc;

struct MyString(String);

impl MyString {
	fn new(s: &str) -> MyString {
		MyString(s.to_string())
	}
}

fn main() {
	let mut threads = Vec::new();
	//cant use Rc for shared references - not thread safe
	let name = Arc::new(MyString::new("dolly"));

	for i in 0..5 {
		//note that MyString can't be cloned - but the reference can!
		let tname = name.clone();
		let t = thread::spawn(move || {
			println!("hello {} count {}", tname.0, i);
		});
		threads.push(t);
	}

	for t in threads {
		t.join().expect("thead failed");
	}
}