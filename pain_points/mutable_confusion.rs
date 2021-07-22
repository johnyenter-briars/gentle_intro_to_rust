use std::collections::HashMap;

fn main() {
	let mut m = HashMap::new();
	
	m.insert("one", 1);
	m.insert("two", 2);

	if let Some(r) = m.get_mut("one") {
		*r = 10;
	} else {
		m.insert("one", 1);
	}
}