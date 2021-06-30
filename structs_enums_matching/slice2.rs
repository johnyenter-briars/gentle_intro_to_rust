fn main(){
	let ints = [1, 2, 3, 4, 5];
	let slice = &ints;
	let first = slice.get(0);
	let last = slice.get(5); //returns the Option type

	println!("first: {:?}", first);
	println!("last: {:?}", last);
}