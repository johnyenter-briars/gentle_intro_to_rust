extern crate chrono;
use chrono::*;

fn main() {
    let date = Local.ymd(2012, 3, 14);
    println!("date: {:?}", date);

}
