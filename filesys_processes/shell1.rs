use std::process::Command;

fn shell(cmd: &str) -> (String, bool) {
	let cmd = format!("{} 2>&1", cmd);
	let shell = if cfg!(windows) {"cmd.exe"} else {"/bin/sh"};

	let flag = if cfg!(windows) {"/c"} else {"-c"};

	let output = Command::new(shell)
		.arg(flag)
		.arg(&cmd)
		.output()
		.expect("no shell?");
	(
		String::from_utf8_lossy(&output.stdout).trim_end().to_string(),
		output.status.success()
	)

}

fn shell_success(cmd: &str) -> Option<String> {
	let (output, success) = shell(cmd);
	if success {Some(output)} else {None}
}

fn main() {
	println!("Shell output: {:?}", shell_success("echo 'hello world'"));
}