#[macro_use]
extern crate nom;

named!(get_greeting<&str,&str>,
    ws!(tag!("hi"))
);

fn main() {
    let res = get_greeting("hi there");
    println!("{:?}", res);
}
