struct Point {
	x: f32,
	y: f32
}

fn match_tuple(t: (i32, String)) {
	let text = match t {
		(0, s) => format!("zero {}", s),
		(1, ref s) if s == "hello" => format!("hello one!"),
		//cant match against (1, "hello") cause "hello" is of type &'static str
		//if type was &str we could match directly
		tt => format!("no match? {:?}", tt),
		//or say _ => ... if you're not interested in the value

	};
	println!("{}", text);
}


fn main() {
	let t = (10, "hello".to_string());

	// let (n, s) = t; //t has been moved - it is no more

	let (ref n, ref s) = t; //n and s are borrowed from t
	//n is &i32 and s is &String
	
	let p = Point { x: 1.0, y: 2.0};
	let Point {x, y} = p; //p still lives since x and y implement copy


	let ot = Some((2, "hello".to_string()));

	if let Some((_, ref s)) = ot {
		assert_eq!(s, "hello");
	} //we just borrowed the string, no 'destructive destructuring'

	//parse nees to workout its return type from context
	if let Ok(n) = "42".parse::<i32>() { //turbofish syntax
		println!("n is: {}", n);

	}






}