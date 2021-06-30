#[derive(Debug)]
struct Person {
	first_name: String,
	last_name: String
}

impl Person {
	fn new(first: &str, last: &str) -> Person{
		Person{
			first_name: first.to_string(),
			last_name: last.to_string()
		}
	}

	fn full_name(&self) -> String {
		format!("{} - {}", self.first_name, self.last_name)
	}

	fn copy(&self) -> Self {
		Self::new(&self.first_name, &self.last_name)
	}

	fn set_first_name(&mut self, name: &str) {
		self.first_name = name.to_string();
	}

	//Data is MOVING when no & is specified from self
	fn to_tuple(self) -> (String, String) {
		(self.first_name, self.last_name)
	}
}

fn main() {
	let p = Person{
		first_name: "John".to_string(),
		last_name: "Smith".to_string()
	};

	let x = vec![10, 20, 30];

	println!("person: {} {}", p.first_name, p.last_name);

	let p = Person::new("John", "Whatchamajinger");

	println!("person: {} {}", p.first_name, p.last_name);

	println!("full name {}", p.full_name());

	let mut newp = p.copy();
	newp.set_first_name("David");

	println!("full name {}", p.full_name());
	println!("full name {}", newp.full_name());

	println!("Debug dump {:?}", newp);

	println!("This action will MOVE the value: {:?}", newp.to_tuple());

	// println!("This action won't work - value has moved {:?}", newp);

}