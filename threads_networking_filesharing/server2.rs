use std::net::{TcpStream, TcpListener};
use std::io::prelude::*;
use std::io;

fn handle_connection(stream: TcpStream) -> io::Result<()> {
	let mut reader = io::BufReader::new(stream);

	let mut text = String::new();

	reader.read_line(&mut text)?;
	println!("got {}", text.trim_right());
	Ok(())
}

fn main() {
	let listener = TcpListener::bind("127.0.0.1:8000").expect("could not start server!");

	//accept connections and get the tcpstream
	for connection in listener.incoming() {
		match connection {
			Ok(stream) => {
				if let Err(e) = handle_connection(stream) {
					println!("error? {:?}", e);
				}

			},
			Err(e) => {
				println!("connection failed {}", e);
			}
		}
	}
}
