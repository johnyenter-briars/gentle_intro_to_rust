fn dump(arr: &[i32]) {
	println!("array is: {:?}", arr);
}


fn main(){
	let mut v = Vec::new(); //allocates from the heap and becomes the "owner of that memory"

	v.push(10);
	v.push(20);
	v.push(30);

	let first = v[0]; //panic if out of range
	let maybe_first = v.get(0);

	println!("v is {:?}", v);
	println!("first is {:?}", first);
	println!("maybefirst is {:?}", maybe_first);
	
	dump(&v);

	let slice = &v[1..]; //& is coercing the vector into a slice
	println!("slice is {:?}", slice);

}