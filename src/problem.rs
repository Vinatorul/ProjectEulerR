use std::fs::File;
use std::io::{BufRead, BufReader};

struct Problem {
	id: u32,
	statement: Vec<String>,
	name: Vec<String>,
	authors: Vec<String>,
	input: Vec<String>
}

enum State {
	Default,
	Name,
	Statement,
	Authors,
	Input
}

impl Problem {
	fn new(id: u32) -> Problem {
		let reader = match File::open(format!("problems/{}.rs", id)) {
				Ok(file) => file,
				Err(err) => panic!("Unable to load problem '#{}`: {}", id, err),
			};
		let buf = BufReader::new(&reader);
		let mut statement = vec![];
		let mut name = vec![];
		let mut authors = vec![];
		let mut input = vec![];
	    let mut state = State::Default;
	    let mut info = false;
	    for line in buf.lines()  {
	    	let l = line.unwrap();
	    	if !info {
	    		if l == "/* [info]" {
		    		info = true;
	    		}
		    	continue;
	    	}
	        match l.as_ref() {
	        	"[authors]" => state = State::Authors,
	        	"[name]" => state = State::Name,
	        	"[statement]" => state = State::Statement,
	        	"[input]" => state = State::Input,
	        	"[end] */" => break,
	        	_ => {
	        		match state {
	        			State::Authors => authors.push(l.trim().to_string()),
	        			State::Name => name.push(l.trim().to_string()),
	        			State::Statement => statement.push(l.trim().to_string()),
	        			State::Input => input.push(l.trim().to_string()),
	        			_ => continue
	        		}
	        	}
	        }
	    } 
		Problem {
			id: id,
			statement: statement,
			name: name,
			authors: authors,
			input: input,
		}
	}
}

#[test]
fn read_test() {
	let p = Problem::new(0);
	assert_eq!(p.id, 0);
	assert_eq!(p.authors, vec!["Testhor", "qwerty"]);
	assert_eq!(p.name, vec!["Test name", "check"]);
	assert_eq!(p.statement, vec!["Test statement", "3", "4"]);
	assert_eq!(p.input, vec!["1sd", "check", "123"]);
}