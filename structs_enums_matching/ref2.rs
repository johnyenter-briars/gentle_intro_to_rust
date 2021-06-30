fn main() {
	let s1 = "hello world".to_string();
	let mut rs1 = &s1;
	{
		let tmp = "testttt".to_string();
		rs1 = &tmp;
	}
	println!("ref: {}", rs1);
}