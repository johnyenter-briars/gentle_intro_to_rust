//how to do a struct with a reference to itselt?
//rust cant do NULL pointers
//need to use box

//a box contains an allocated pointer to data and always has a fixed size
type NodeBox = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
	payload: String,
	left: NodeBox,
	right: NodeBox
}

impl Node {
	fn new(s: &str) -> Node{
		Node {payload: s.to_string(), left: None, right: None}
	}

	fn boxer(node: Node) -> NodeBox{
		Some(Box::new(node))
	}

	fn set_left(&mut self, node: Node) {
		self.left = Self::boxer(node);
	}

	fn set_right(&mut self, node: Node) {
		self.right = Self::boxer(node);
	}

	//note box is a smart pointer - no need to call unbox in order to call node methods
	fn insert(&mut self, data: &str) {
		if data < &self.payload {
			match self.left {
				Some(ref mut n) => n.insert(data),
				None => self.set_left(Self::new(data))
			}
		} else {
			match self.right {
				Some(ref mut n) => n.insert(data),
				None => self.set_right(Self::new(data))
			}
		}
	}

	//in order traversal
	fn visit(&self) {
		if let Some(ref left) = self.left {
			left.visit();
		}
		println!("'{}'", self.payload);
		if let Some(ref right) = self.right {
			right.visit();
		}
	}
}

fn main() {
	let mut root = Node::new("root");
	// root.set_left(Node::new("left"));
	// root.set_right(Node::new("right"));
	root.insert("one");
	root.insert("two");
	root.insert("three");

	println!("arr {:#?}", root); //the # means extended

	root.visit();
}


