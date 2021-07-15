fn main() {

	let mut v = vec![10, 20, 30, 40];
	//no search method for finding index of element

	assert_eq!(v.iter().position(|&i| i == 30).unwrap(), 2);

	v.extend([60, 70, 80].iter());

	let mut strings = vec!["hello".to_string(), "dolly".to_string()];

	strings.extend(["you", "are", "fine"].iter().map(|&s| s.to_string()));

	println!("{:?}", strings);

	//three kinds of iterators

	// for x in v {} //returns T, consumes v
	// for x in &v {} //returns &T
	// for x in &mut v {} //returns &mut T

	//handle errors in an iterator

	let nums = ["5", "52", "john"];

	let iter = nums.iter().map(|s| s.parse::<i32>());
	// let converted: Vec<_> = iter.collect();

	//automatically unwrap - anf if there is an error - will return the first one
	let better_converted: Result<Vec<_>, _> = iter.collect();

	println!("{:?}", better_converted);
}