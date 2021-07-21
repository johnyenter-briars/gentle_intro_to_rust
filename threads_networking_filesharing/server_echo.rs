use std::net::{TcpStream, TcpListener};
use std::io::prelude::*;
use std::io;

fn handle_connection(stream: TcpStream) -> io::Result<()> {
	//bufreader will eventually consume the stream
	//need to clone its reference (still refere to same underlying socker)
	let mut ostream = stream.try_clone()?;

	let mut reader = io::BufReader::new(stream);
	
	let mut text = String::new();
	reader.read_line(&mut text)?;

	ostream.write_all(text.as_bytes())?;

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

