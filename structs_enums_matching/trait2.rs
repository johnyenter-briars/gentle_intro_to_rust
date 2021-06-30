use std::fmt;

struct Person {
    first_name: String,
    last_name: String
}

impl Person {

    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: name.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}",self.first_name, self.last_name)
    }

    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }

    fn to_tuple(self) -> (String,String) {
        (self.first_name, self.last_name)
    }
}

impl fmt::Debug for Person {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Debug implementation: {}", self.full_name())
	}
}

fn main() {
	let p = Person::new("john", "smith");

	println!("{:?}", p);
}