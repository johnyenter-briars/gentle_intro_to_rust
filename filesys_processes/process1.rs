use std::process::Command;

/*
    program didn't exist, was bad, or we were not allowed to run it
    program ran, but was not successful - non-zero exit code
    program ran, with zero exit code. Success!
*/
fn main() {
	let status = Command::new("rustc")
		.arg("-V")
		.status() //causes it to run - returns a result wihch is either Ok if program ran
		.expect("no rustc?");
		//by default - program's output goes to standard out 

	//call unwrap - cause we ant get the code if the program was killed by a signal
	println!("cool {} code {}", status.success(), status.code().unwrap());

	//as with status. output blocks until the child process is finished
	let output = Command::new("rustc")
		.arg("-V")
		.output() 
		.expect("no rustc?");
		//by default - program's output goes to standard out 

	if output.status.success() {
		println!("ok!");
	}

	println!("len stdout {}, stderr {}", output.stdout.len(), output.stderr.len());
}