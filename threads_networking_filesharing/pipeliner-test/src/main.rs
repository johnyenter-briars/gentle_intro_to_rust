extern crate pipeliner;
use pipeliner::Pipeline;
use std::process::Command;
use std::net::*;

fn shell(cmd: &str) -> (String,bool) {
    let cmd = format!("{} 2>&1",cmd);
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("no shell?");
    (
        String::from_utf8_lossy(&output.stdout).trim_right().to_string(),
        output.status.success()
    )
}

fn main() {
    let addresses: Vec<_> = (1..40).map(|n| format!("192.168.0.{}:0", n)).collect();
    let n = addresses.len();

    // for result in addresses.with_threads(n).map(|s| shell(&s)) {
    //     if result.1 {
    //         println!("got: {}\n", result.0);
    //     }
    // }
    
    for result in addresses.with_threads(n).map(|s| s.to_socket_addrs().expect("Could not convert to socket address")) {
        println!("got: {:?}", result);
    }

}
