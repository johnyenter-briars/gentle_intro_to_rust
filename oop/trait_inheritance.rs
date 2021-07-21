macro_rules! dbg {
	($x:expr) => {
		println!("{} = {:?}", stringify!($x), $x)
	}
}
trait Show {
	fn show(&self) -> String;
}

trait Location {
	fn location(&self) -> String;
}

//combines the two traits into one!
trait ShowTell: Show + Location {}

#[derive(Debug)]
struct Foo {
	name: String,
	location: String
}

impl Foo {
	fn new(name: &str, location: &str) -> Foo {
		Foo {
			name: name.to_string(),
			location: location.to_string()
		}
	}
}

impl Show for Foo {
	fn show(&self) -> String {
		self.name.clone()
	}
}

impl Location for Foo {
	fn location(&self) -> String {
		self.location.clone()
	}
}

//note - no defitiont needed
impl ShowTell for Foo {}

fn main() {
	let foo = Foo::new("john", "house");

	dbg!(foo.show());
	dbg!(foo.location());

	let st: &ShowTell = &foo;

	dbg!(st.show());
	dbg!(st.location());

}

