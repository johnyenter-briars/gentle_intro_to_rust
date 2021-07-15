use std::collections::HashMap;

fn main() {
	let entries = [("one","eins"),("two","zwei"),("three","drei")];

	if let Some(val) = entries.iter().find(|t| t.0 == "two") {
		assert_eq!(val.1, "zwei");
	}

	let mut map = HashMap::new();

	map.insert("one","eins");
	map.insert("two","zwei");
	map.insert("three","drei");

	assert_eq!(map.contains_key("two"), true);
	assert_eq!(map.get("two"), Some(&"zwei")); //get returns referenc to value

	let mut map = HashMap::new();
	map.insert("one",1);
	map.insert("two",2);
	map.insert("three",3);

	println!("before {}", map.get("two").unwrap());

	let mref = map.get_mut("two").unwrap();
	*mref = 20;

	println!("after {}", map.get("two").unwrap());

	//more common way to unwrap
	if let Some(v) = map.get("two") {
		let res = v +1;
		assert_eq!(res, 21);
	}
	match map.get_mut("two") {
		Some(mref) => *mref=20,
		None => panic!("now we panic!")
	};

	for (k, v) in map.iter() {
		println!("key {} value {}", k, v);
	}

}