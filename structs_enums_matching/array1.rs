//read as "slice of i32"
fn sum(values: &[i32]) -> i32 {
	let mut res = 0;
	for i in 0..values.len(){
		res += values[i];
	}
	res
} 


fn main() {

	let arr: [i32; 4] = [10, 20, 30, 40];
	let first = arr[0];
	println!("first: {}", first);

	for i in 0..4 {
		println!("element: {}", arr[i]);
	}

	println!("length {}", arr.len());

	let arr2: [i32; 4] = [10, 20, 30, 40];
	//"borrow" not "address of"
	let res = sum(&arr2); //passing it a slice of arr2
	println!("sum: {}", res);

}