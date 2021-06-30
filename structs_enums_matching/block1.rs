fn main(){
	let a = 10;
	let b = "hello";
	{
		let c = "hello".to_string();
		//a, b, and c are still visible
	}
	//c is dropped (resources reclaimed)
	for i in 1..a{
		let b = &b[1..i];
		//original b is no longer visible - it is shadowed
		println!("test: {}", b);
	}
	//slice b is droppped
	// i is not visible
}