//rust implicetly creates the signature as
//fn pair<'a>(s: &'a str, ch: char) -> (&'a str, &'a str)
//output strings live AT LEAST AS LONG AS input string
fn pair(s: &str, ch: char) -> (&str, &str) {
	if let Some(index) = s.find(ch) {
		(&s[0..index], &s[index+1..])
	} else {
		(s, "")
	}
}

fn main() {
	let (left, right) = pair("hello:world", ':');
	println!("{} {}", left, right);
}