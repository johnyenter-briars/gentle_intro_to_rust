fn dump<T> (value: &T)  
where T: std::fmt::Debug{ //rustc needs to be told that T implements Debug
	println!("value is {:?}", value);
}

struct Foo {
	name: String
}
// stil wont work cause x * x is of type T::Output not T
//which makse sense - no gaurentee type of T is same as type T * T
// fn sqrt<T> (x: T) -> T
// where T: std::ops::Mul {
// 	x * x
// }

fn sqrt<T> (x: T) -> T::Output
//need to contrain the type to implement copy as well. After all, all values are moved. So the type T (probably an integer) needs to be able to copy itself in order to return the value
where T: std::ops::Mul + Copy{ 
	x * x
}


fn main() {
	let n = 42;
	dump(&n);

	let f = Foo { name: "john".to_string() };
	// dump(&f); //compiler error - since foo doesnt implemnt debug

	let res = sqrt(10.0);
	println!("res{}", res);
}