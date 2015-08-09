use std::fs::File;
use std::io::{BufRead, BufReader};

struct Problem {
	id: u32,
	statement: String,
	name: String,
}

enum State {
	Default,
	Name,
	Statement,
}

impl Problem {
	fn new(id: u32) -> Problem {
		let reader = match File::open(format!("problems/{}.rs", id)) {
				Ok(file) => file,
				Err(err) => panic!("Unable to load problem '#{}`: {}", id, err),
			};
		let buf = BufReader::new(&reader);
		let mut statement = String::new();
		let mut name = String::new();
	    let mut state = State::Default;
	    for line in buf.lines()  {
	    	let l = line.unwrap();
	        match l.as_ref() {
	        	"/// [name]" => state = State::Name,
	        	"/// [statement]" => state = State::Statement,
	        	"/// [end]" => break,
	        	_ => {
	        		match state {
	        			State::Name => {
	        				if !name.is_empty() {
	        					name.push('\n');
	        				}
	        				name.push_str(&l[3..].trim())
	        			},
	        			State::Statement => {
	        				if !statement.is_empty() {
	        					statement.push('\n');
	        				}
	        				statement.push_str(&l[3..].trim())
	        			},
	        			_ => continue
	        		}
	        	}
	        }
	    } 
		Problem {
			id: id,
			statement: statement,
			name: name,
		}
	}
}

#[test]
fn read_test() {
	let p = Problem::new(0);
	assert_eq!(p.name, "Test name\ncheck");
	assert_eq!(p.statement, "Test statement\n3\n4");
}