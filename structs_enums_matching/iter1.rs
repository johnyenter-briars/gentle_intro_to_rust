fn main() {
	let mut iter = 0..3;

	assert_eq!(iter.next(), Some(0));
	assert_eq!(iter.next(), Some(1));
	assert_eq!(iter.next(), Some(2));
	assert_eq!(iter.next(), None);

	//faster to use iterators then indexes
	for i in [10, 20, 30].iter() {
		println!("{}", i);
	}

	//slices converted implicetly to iterators
	for i in &[10, 20, 30] {
		println!("{}", i);
	}

	let sum: i32 = (0..6).sum();
	println!("sum: {}", sum);
	let sum: i64 = [1, 2, 3, 4].iter().sum();
	println!("sum: {}", sum);
}