use std::f64::consts;
fn main(){
	let pi: f64  = 3.1416;
	let x = pi/2.0;
	let cosine = x.cos();

	println!("cosine: {}", cosine);
	let pi2: f64  = consts::PI;
	let y = 2.0 * pi2;
	let abs_diff = (y.cos() -1.0).abs();
	assert!(abs_diff < 1e-10);
}