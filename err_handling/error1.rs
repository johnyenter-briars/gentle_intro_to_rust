use std::fmt;
use std::error::Error;
use std::error;
use std::num::ParseFloatError;

#[derive(Debug)]
struct MyError {
	details: String
}

impl MyError {	
	fn new(msg: &str) -> MyError {
		MyError{details: msg.to_string()}
	}

}
impl fmt::Display for MyError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "data: {}", self.details)
	}
}

impl error::Error for MyError {
	fn description(&self) -> &str {
		&self.details
	}
}

impl From<ParseFloatError> for MyError {
	fn from(err: ParseFloatError) -> Self {
		MyError::new(err.description())
	}
}

fn raises_my_error(yes: bool) -> Result<(), MyError> {
	if yes {
		Err(MyError::new("borked"))
	} else {
		Ok(())
	}
}

fn parse_64(s: &str, yes: bool) -> Result<f64, MyError> {
	raises_my_error(yes)?;
	let x: f64 = s.parse()?;
	Ok(x)
}

fn main() {
	let res = raises_my_error(true);
	println!("{:?}", res);

	println!("{:?}", parse_64("42", true));
	println!("{:?}", parse_64("?42", false));
}