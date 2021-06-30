fn add_mul(x: f64, y: f64) -> (f64, f64) {
	(x+y, x*y)
}

fn main() {
	let tuple = ("hello", 5, 'c');
	println!("second element {}", tuple.1);
	
	for t in ["zero", "one", "two"].iter().enumerate() {
		println!("{} {}", t.0, t.1);
	}

	let names = ["ten", "hundred", "thousand"];
	let nums = [10, 100, 1000];
	
	for p in names.iter().zip(nums.iter()) {
		println!("{} {}", p.0, p.1);
	}


}