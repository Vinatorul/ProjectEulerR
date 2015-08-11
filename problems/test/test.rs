use std::io;

fn main() {
	let mut l1 = String::new();
    io::stdin().read_line(&mut l1).unwrap();
	println!("Hello, {}", l1);
}