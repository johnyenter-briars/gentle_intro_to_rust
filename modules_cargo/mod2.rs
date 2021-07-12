mod foo; //rustc will cause foo to be compiles with mod2
mod boo;

fn main() {
	let f = foo::Foo::new("no need for make files!");
	println!("{:?} ", f);

	let res = boo::answer();

	println!("{}", res);

	println!("{}", boo::bar::question());
}