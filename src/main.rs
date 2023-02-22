
fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() != 2 || args.len() != 4  {
		println!("Incorrect Formar");
		println!("cargo run [k] | cargo run [k] [x] [y]");
		return;
	}



}
