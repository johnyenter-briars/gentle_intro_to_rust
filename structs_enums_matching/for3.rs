fn main(){
	for i in 0..5{
		let even_or_odd = if i % 2 == 0 { "even" } else { "odd" };
		println!("even or odd: {}", even_or_odd);
		
	}
}