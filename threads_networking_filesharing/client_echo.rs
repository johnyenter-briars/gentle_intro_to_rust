use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
	let mut stream = TcpStream::connect("127.0.0.1:8000").expect("connection failed!");
	
	let msg = "hello from client!";

	write!(stream, "{}\n", msg).expect("write failed!");

	let mut response = String::new();

	stream.read_to_string(&mut response).expect("read failed");
	let text = response.trim_end();
	println!("data echoed: {}", text);
	assert_eq!(msg,text);
}