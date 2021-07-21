use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
	let answer = Arc::new(Mutex::new(42));

	let answer_ref = answer.clone();

	let t = thread::spawn(move || {
		//unlock the reference so that the thread can access safly
		let mut answer = answer_ref.lock().unwrap();

		*answer = 55;
	});

	t.join().unwrap();

	let ar = answer.lock().unwrap();

	assert_eq!(*ar, 55);
}