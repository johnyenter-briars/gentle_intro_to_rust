fn dump(s: &str) {
	println!("string: '{}'", s);
}

fn arr_to_string(arr: &[i32]) -> String {
	let mut s = '['.to_string();

	for x in arr {
		s += &x.to_string();
		s.push(',');
	}
	s.pop();
	s.push(']');

	s
}

fn main() {
	let text = "hello world!"; //string slice
	let s = text.to_string(); //allocated string

	dump(text);
	dump(&s);

	let mut s = String::new();
	//initially empty!
	s.push('H');
	s.push_str("ello!");
	s.push(' ');
	s += "World!"; //short for .push_str
	s.pop();

	println!("Strnig: {}", s);

	println!("{}", arr_to_string(&[10, 20, 30]))
}