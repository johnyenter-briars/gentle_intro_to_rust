#[derive(Debug,PartialEq)]
enum Direction {
	Up,
	Down,
	Left,
	Right
}

impl Direction {
	fn as_str(&self) -> &'static str {
		//normally we dont need the dereference pointer
		//normally rust can asume self.first_name without (*self).first_name
		match *self { //*self has type Direction
			Direction::Up => "Up",
			Direction::Down => "Down",
			Direction::Left => "Left",
			Direction::Right => "Right"
		}
	}

	fn next(&self) -> Direction {
		use Direction::*;
		match *self { //*self has type Direction
			Direction::Up => Direction::Right,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
			Direction::Right => Direction::Down
		}
	}
}

fn main() {
	let start = Direction::Left;

	println!("start {:?}", start.as_str()); //note as_str does not allocate

	assert_eq!(start, Direction::Left);

	let mut d = start;
	for _ in 0..8 {
		println!("d : {:?}", d);
		d = d.next();
	}
}