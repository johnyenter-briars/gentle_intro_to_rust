use std::cell::RefCell;

fn main() {
	let greetings = RefCell::new("hello".to_string());

	assert_eq!(*greetings.borrow(), "hello");
	assert_eq!(greetings.borrow().len(), 5); //method calls will dereference for you 

	*greetings.borrow_mut() = "hola".to_string();

	assert_eq!(*greetings.borrow(), "hola");

	let mut gr = greetings.borrow_mut();

	*gr = "hola".to_string();

	assert_eq!(*greetings.borrow(), "hola"); // will blow up - since we alreayd borrowed on line 13
}