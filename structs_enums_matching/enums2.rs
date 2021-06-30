#[derive(PartialEq, PartialOrd)]
enum Speed {
	Slow = 10,
	Medium = 20,
	Fast = 50
}

enum Difficulty {
	Easy = 1,
	Medium,
	Hard
}

fn main() {
	let s = Speed::Slow;

	let speed = s as u32;

	println!("speed: {}", speed);

	let diff = Difficulty::Easy;

	println!("diff: {}", diff as u32);
}