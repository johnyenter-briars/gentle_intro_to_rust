fn main() {
	let m = 2.0;
	let c = 0.5;

	let linear = |x: f64| m*x + c;
	let sc = |x: f64| m*x.cos();

	/*
	both closures implement Fn(x: 64) -> 64
	BUT - they have different types and sizes. 
	to store them in the same vector you need to do 
	Box<Fn(x: 64) -> 64 + 'a>
	*/
	
	let v: Vec<Box<Fn(f64) -> f64>> = vec![Box::new(linear), Box::new(sc)];

	for closure in v.iter() {
		println!("{}", closure(10.0));
	}
}