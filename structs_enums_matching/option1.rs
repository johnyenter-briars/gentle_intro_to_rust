fn main(){
	let ints = [1, 2, 3, 4, 5];
	let slice = &ints;
	let first = slice.get(0);
	let last = slice.get(5); //returns the Option type

	println!("first: {:?}", first);
	println!("last: {:?}", last);

	println!("{}", first.is_some());
	println!("first value: {}", first.unwrap());

	let maybe_last = slice.get(5);
	let last = if maybe_last.is_some() {
		*maybe_last.unwrap() //returned value is &i32 - need to dereference it
	} else {
		-1
	};

	println!("last: {}", last);

	//better way
	let last2 = *slice.get(5).unwrap_or(&-1);
	println!("last: {}", last2);
}