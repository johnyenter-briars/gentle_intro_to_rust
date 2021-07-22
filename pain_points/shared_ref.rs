fn main() {
	let mut answer = Box::new("hello".to_string());

	*answer = "world".to_string();

	answer.push('!'); //no dereference needed for methods

	println!("{} {}", answer, answer.len());
}