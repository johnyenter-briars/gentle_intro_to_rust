//this enum can only be one of these values - 
//its size is the size of the largest value
#[derive(Debug)]
enum Value {
	Number(f64),
	Str(String),
	Bool(bool)
}

//this is cool, but currently the value is MOVED and cant use it after
fn eat_and_dump(v: Value) {
	use Value::*;
	match v {
		Number(n) => println!("number is {}", n),
		Str(s) => println!("string is '{}'", s),
        Bool(b) => println!("boolean is {}", b)
	}
}

fn dump(v: &Value) {
	use Value::*;
	match *v { //type of *V is Value
		Number(n) => println!("number is {}", n),
		// Str(s) => println!("string is '{}'", s), //this wont work, String does not implement Copy
		Str(ref s) => println!("string is '{}'", s),
        Bool(b) => println!("boolean is {}", b)
	}
}

impl Value {
	//this guy is gonna MOVE the value - hense the naming to_str
	fn to_string(self) -> Option<String> {
		match self {
			Value::Str(s) => Some(s),
			_ => None
		}
	}

	//this guy wont move the value 
	fn as_string(self) -> Option<String> {

		if let Value::Str(s) = self {
			Some(s)
		} else {
			None
		}
	}
}


fn main() {
	use Value::*;

	let n = Number(2.3);
	let s = Str("hello".to_string());
	let b = Bool(true);

	println!("n {:?} s {:?} b {:?}", n,s,b);

	eat_and_dump(n);
	// eat_and_dump(s);
	eat_and_dump(b);

	// println!("this wont work: {:?}", n); //compiler error, since function moves the value

	dump(&s);

	println!("s? {:?}", s.to_string());
    // println!("{:?}", s) // error! s has moved...




}