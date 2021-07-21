use std::thread;
use std::sync::mpsc;

fn main() {
	let nthreads = 5;
	let (sender_channel, receiver_channel) = mpsc::channel();

	for i in 0..nthreads {
		let sender_channel = sender_channel.clone();
		thread::spawn(move || {
			let response = format!("hello {}", i);
			sender_channel.send(response).unwrap(); //send never blocks
			
		});
	}

	for _ in 0..nthreads {
		println!("got {:?}", receiver_channel.recv()); //recv will block
	}

	//syn channel DOES block on send
	let (sender_channel, receiver_channel) = mpsc::sync_channel(0);

	let t1 = thread::spawn(move || {
		for i in 0..4 { //notice we only write up to 4
			sender_channel.send(i).unwrap();
		}
	});

	for _ in 0..nthreads {
		println!("got {:?}", receiver_channel.recv().unwrap()); //recv will block
	}
}
