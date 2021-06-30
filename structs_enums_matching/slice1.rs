fn main() {
	let ints = [1, 2, 3];
	let floats = [1.1, 2.1, 3.1];
	let strings = ["hello", "world"];
	let ints_ints= [[1, 2], [3, 4]];

	//slices borrowing data
	let slice1 = &ints[0..2];
	let slice2 = &ints[3..];


	println!("{:?}", slice1); //{:?} debug symbol
	println!("{:?}", slice2); //{:?} debug symbol
}