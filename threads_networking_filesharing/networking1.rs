use std::net::*;

fn main() {
	for res in "google.com:80".to_socket_addrs().expect("bad") {
		println!("got: {}", res);
	}
}