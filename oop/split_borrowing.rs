struct Foo {
	one: String,
	two: String
}

impl Foo {
	fn borrow_one_mut(&mut self) -> &mut String {
		*mut self.one
	}

	fn borrow_both(&mut self) -> (&str, &str) {
		(&self.one, &self.two)
	}
}