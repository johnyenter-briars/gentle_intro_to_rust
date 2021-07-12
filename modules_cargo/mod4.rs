//rustc mod4.rs --extern foo=libfoo.rlib
extern crate foo;

fn main() {
    let f = foo::Foo::new("hello");
    println!("{:?}", f);
}