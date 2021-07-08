use std::env;
use std::path::Path;
use std::fs;
use std::io;

fn dump_dir(dir: &str) -> io::Result<()> {
	for entry in fs::read_dir(dir)? {
		let entry = entry?;
		let data = entry.metadata()?;
		let path = entry.path();
		if data.is_file() {
			if let Some(ex) = path.extension() {
				if ex == "rs" && data.len() > 1024 {
					println!("{} length {}", path.display(), data.len());
				}
			}

		}
	}
	Ok(())
}
fn main() {
	let file = env::args().skip(1).next().unwrap_or("file6.rs".to_string());
	let path = Path::new(&file);
	match path.metadata() {
		Ok(data) => {
			println!("type: {:?}", data.file_type());
			println!("len: {:?}", data.len());
			println!("perm: {:?}", data.permissions());
			println!("modified: {:?}", data.modified());
		},
		Err(e) => println!("error: {:?}", e)
	}

	let _result = dump_dir(".");
}