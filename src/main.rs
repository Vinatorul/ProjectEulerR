use std::env;

mod problem;

fn main() {
	let args: Vec<String> = env::args().collect();

	println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}
