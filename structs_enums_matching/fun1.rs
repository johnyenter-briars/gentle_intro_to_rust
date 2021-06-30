fn abs(x: f64) -> f64 {
	if x > 0.0 {
		x
	} else {
		-x
	}
}


fn sqr(x: f64) -> f64{
	x * x
}

fn main(){
	let res = sqr(5.2);
	let abs_res = abs(-5.2);

	println!("Sqrt root: {}", res);
}