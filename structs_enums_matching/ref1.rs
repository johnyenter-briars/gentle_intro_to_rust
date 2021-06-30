fn by_ref(x: &i32) -> i32 {
	*x + 1
}

fn modifies(x: &mut f64) {
	*x = 1.0;
}

fn main() {
	let i = 10;
	let res1 = by_ref(&i);
	let res2 = by_ref(&42);
	println!("{} {}", res1, res2);

	let mut res3 = 0.0;
	modifies(&mut res3);
	println!("{}", res3);
}