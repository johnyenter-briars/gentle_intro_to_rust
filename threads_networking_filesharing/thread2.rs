use std::thread;

fn main() {
	let t = thread::spawn(|| {
		println!("hellp");
		panic!("i give up!");
	});

	println!("wait {:?}", t.join());
}
