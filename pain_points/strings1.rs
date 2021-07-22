fn main() {
	let s1 = "hello".to_string();
    let s2 = s1.clone();
    assert!(s1 == s2);  // works
    assert!(s1 == "hello"); // works
    // assert!(s1 == &s2); // doesnt work - reference and value not the same type

	// let s3 = s1 + s2;  //cant concatinate - need to use format!
	let s = "test,one,two,tthree,".to_string();
	let parts: Vec<_> = s.split(',').collect();
	//returns vector where the parts are borrowed from the origional string
	println!("parts: {:?}", parts);

	

}