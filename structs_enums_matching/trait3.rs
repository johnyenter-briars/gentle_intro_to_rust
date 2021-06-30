struct FRange {
	val: f64,
	end: f64,
	incr: f64
}

fn range(x1: f64, x2: f64, skip: f64) -> FRange {
	FRange { val: x1, end: x2, incr: skip }
}

impl Iterator for FRange {
	type Item = f64;

	fn next(&mut self) -> Option<Self::Item> {
		let res = self.val;
		if res >= self.end {
			None
		} else {
			self.val += self.incr;
			Some(res)
		}
	}
}

fn main() {
	for x in range(0.0, 1.0, 0.1) {
		println!("{:.1}", x);
	}

	//because we implemented the Iterator trait on FRange, we still get all of the default iterator methods
	//like collect and map and so forth
	let v: Vec<f64> = range(0.0, 1.0, 0.1).map(|x| x.sin()).collect();

	println!("values: {:?}", v);
}