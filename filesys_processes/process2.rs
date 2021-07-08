use std::process::{Command, Stdio};

fn main() {
	let mut child = Command::new("rustc")
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn() //spawn subprocess emmediatly
		.expect("no rustc?");
		//redirecting output - equivant to: > /dev/null 2> /dev/null
		//could also use .stdout(Stdio::piped()) - redirected to pipe
	
	//can do other stuff in the meantime!

	let res = child.wait(); // wait for the child to finish

	//can also use wait_with_output to get Result<Output> to get piped output/intput/err

	//child.kill() does what it says on the tin :)

	println!("res: {:?}", res);
}