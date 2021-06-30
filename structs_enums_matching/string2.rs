fn main() {
	let text = "the red fox and the lazy dog";//string slice
	let words: Vec<&str> = text.split_whitespace().collect();
	println!("words: {:?}", words);

	let mut words = Vec::new();
	words.extend(text.split_whitespace());
	println!("words: {:?}", words);

	let stripped: String = text.chars().filter(|ch| ! ch.is_whitespace()).collect();

	println!("stripped: {:?}", stripped);
}