fn good_or_bad(good: bool) -> Result<i32, String> {
	if good {
		Ok(42)
	} else{
		Err("bad".to_string())
	}
}

fn main() {
	println!("{:?}", good_or_bad(true));
	println!("{:?}", good_or_bad(false));

	match good_or_bad(true){
		Ok(n) => println!("we got an: {}", n),
		Err(e) => println!("message: {}", e),
	};
}