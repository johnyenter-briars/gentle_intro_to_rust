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
	//box acts like a smart pointer
	let answer = Box::new(42);
	let maybe_pi = Box::new(3.14);

	//can give this vector away without having to worry about borrowed references
	let show_list: Vec<Box<Show>> = vec![answer, maybe_pi];

	for d in &show_list {
		println!("show {}", d.show());
	}
}
