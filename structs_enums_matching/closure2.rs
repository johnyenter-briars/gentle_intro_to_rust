fn mutate<F>(mut f: F)
where F: FnMut() {
	f()
}

fn main() {

	//can mutate captured references
    let mut s = "world";
    mutate(|| s = "hello");
    assert_eq!(s, "hello");

	let mut s = "world";

	//closure does a mutable borrow of s
	let mut changer = || s = "WORLD";

	changer();
	//does an immutable borrow of s
	assert_eq!(s, "WORLD");
}