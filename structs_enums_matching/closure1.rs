//under the hood, closures look liek this (notice they implement call)
struct MyAnonymousClosure1<'a> {
    m: &'a f64,
    c: &'a f64
}

impl <'a>MyAnonymousClosure1<'a> {
    fn call(&self, x: f64) -> f64 {
        self.m * x  + self.c
    }
}

//can know the generic constraint of a closure
fn apply<F>(x: f64, f: F) -> f64
where F: Fn(f64)->f64{
	f(x)
}

fn main() {
	let f = |x| x * x;

	let res = f(10);

	println!("res {}", res);

	// let res = f(10.0);// error - rust has already decided that the closure takes in an int
	println!("res {}", res);

	let m = 2.0; 
	let c = 1.0;

	let lin = |x| m*x + c;

	println!("res {} {}", lin(1.0), lin(2.0)); //lin closure has BORROWED m and n from its enclosing scope

	let res1 = apply(3.0, lin);
	let res2 = apply(8.0, |x| x.sin());

	let l = lin;

	// println!("using apply function: {}", apply(8.0, |x| x.sin()));



}