//acting like an interface
trait Show {
	fn show(&self) -> String;
}

impl Show for i32 {
	fn show(&self) -> String {
		format!("four-byte signed {}", self)
	}
}

impl Show for f64 {
	fn show(&self) -> String {
		format!("eight-byte float {}", self)
	}
}



fn main() {
	let answer = 42;
	let maybe_pi = 3.14;

	//these references called trait objects
	let v: Vec<&Show> = vec![&answer, &maybe_pi];

	for d in v.iter() {
		println!("show {}", d.show());
	}
}