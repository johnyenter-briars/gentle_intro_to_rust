use std::collections::HashMap;
use std::io;
use std::error;

type CliResult = Result<String, String>;

struct Cli<'a, D> {
	data: D,
	callbacks: HashMap<String, Box<Fn(&mut D, &[&str])->CliResult + 'a>>
}

impl <'a, D: Sized> Cli<'a, D> {
	fn new(data: D) -> Cli<'a,D> {
		Cli{data: data, callbacks: HashMap::new()}
	}

	fn cmd<F>(&mut self, name: &str, callback: F) 
	//Fn means that closure BORROWS from environment - but cant modify
	where F: Fn(&mut D, &[&str])->CliResult + 'a {
		self.callbacks.insert(name.to_string(), Box::new(callback));
	}

	fn proces(&mut self, line: &str) -> CliResult {
		let parts: Vec<_> = line.split_whitespace().collect();
		if parts.len() == 0 {
			return Ok("".to_string());
		}
		match self.callbacks.get(parts[0]) {
			Some(callback) => callback(&mut self.data, &parts[1..]),
			None => Err("no such command".to_string())
		}
	}

	fn go(&mut self) {
		let mut buff = String::new();
		while io::stdin().read_line(&mut buff).expect("error") > 0 {
			let line = buff.trim_start();
			let res = self.proces(line);
			println!("{:?}", res);
			buff.clear();
		}
	}
}

fn ok<T: ToString>(s: T) -> CliResult {
	Ok(s.to_string())
}

fn err<T: ToString>(s: T) -> CliResult {
	Err(s.to_string())
}

fn main() {
	// let mut v: Vec<Box<Fn(f64)->f64>> = Vec::new(); //rustc will read the lambda type as too specific


	// v.push(Box::new(|x| x * x));
	// v.push(Box::new(|x| x / 2.0));

	// for f in v.iter() {
	// 	let res = f(1.0);
	// 	println!("res {}", res);
	// }


	//using the command_peocessing api
	println!("Welcome to the Interactive Prompt! ");

	struct Data {
		answer: i32
	}

	let mut cli = Cli::new(Data{answer: 42});

	cli.cmd("go", |data,args| {
		if args.len() == 0 { return err("need 1 argument!") }

		data.answer = match args[0].parse::<i32>() {
			Ok(n) => n,
			Err(e) => return err(e)
		};

		println!("got : {:?}", args);
		ok(data.answer)
	});

	cli.cmd("show", |data,_| {
		ok(data.answer)
	});

	cli.go();
}