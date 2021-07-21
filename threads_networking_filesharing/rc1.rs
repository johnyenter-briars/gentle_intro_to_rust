use std::rc::Rc;

fn main() {
	let s = "hello dolly".to_string();

	let rs1 = Rc::new(s); //s moves to heap, ref count 1;
	let rs2 = rs1.clone(); // ref count 2

	println!("{} {}", rs1.len(), rs2.len());

}