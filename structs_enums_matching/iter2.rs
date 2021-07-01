fn main() {
	//explicit

	for s in vec.iter() {...} // &String
	for s in vec.iter_mut() {...} // &mut String
	//consumes the value -no longer avil
	for s in vec.into_iter() {...} // String

	// implicit!

	//usually what you want
	for s in &vec {...} // &String
	for s in &mut vec {...} // &mut String
	for s in vec {...} // String

	for n in vec.iter().map(|x: &String| x.len()) {...} //n is usize
	for s in vec.iter().filter(|x: &String| x.len() > 2) {...} //s is &String

	for s in vec.iter().filter(|x: &&String| *x == "one") {...}
	// same as implicit form:
	for s in vec.iter().filter(|x| *x == "one") {...}

	for s in vec.iter().filter(|&x| x == "one") {...} //s is now &String
}