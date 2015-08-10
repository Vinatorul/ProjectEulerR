use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

const PROBLEMS_DIR: &'static str = "problems";

struct Problem {
	str_id: String,
	statement: Vec<String>,
	name: Vec<String>,
	authors: Vec<String>,
}

enum State {
	Default,
	Name,
	Statement,
	Authors,
}

impl Problem {
	fn new(str_id: String) -> Problem {
		let reader = match File::open(format!("{}/{}/info.txt", PROBLEMS_DIR, str_id)) {
			Ok(file) => file,
			Err(err) => panic!("Unable to load problem '{}`: {}", str_id, err),
		};
		let buf = BufReader::new(&reader);
		let mut statement = vec![];
		let mut name = vec![];
		let mut authors = vec![];
		let mut state = State::Default;
		for line in buf.lines()  {
			let l = line.unwrap();
			match l.as_ref() {
				"[authors]" => state = State::Authors,
				"[name]" => state = State::Name,
				"[statement]" => state = State::Statement,
				"[end]" => break,
				_ => {
					match state {
						State::Authors => authors.push(l.trim().to_string()),
						State::Name => name.push(l.trim().to_string()),
						State::Statement => statement.push(l.trim().to_string()),
						_ => continue
					}
				}
			}
		}
		Problem {
			str_id: str_id,
			statement: statement,
			name: name,
			authors: authors,
		}
	}

	pub fn run(&self) {
		let builder = Command::new("rustc")
			.arg(format!("{}/{}/{}.rs", PROBLEMS_DIR, self.str_id, self.str_id))
			.arg("--out-dir")
			.arg(format!("{}/{}", PROBLEMS_DIR, self.str_id))
			.output()
			.unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

		if builder.status.success() {
			let problem = Command::new(format!("{}/{}/{}", PROBLEMS_DIR, self.str_id, self.str_id))
				.output()
				.unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
			let s = String::from_utf8_lossy(&problem.stdout);
			println!("{}", s);
		} else {
			let s = String::from_utf8_lossy(&builder.stderr);
			panic!("rustc failed to build problem {}:\n{}", self.str_id, s);
		}
	}
}

#[test]
fn read_test() {
	let p = Problem::new("test".to_string());
	assert_eq!(p.str_id, "test");
	assert_eq!(p.authors, vec!["Testhor", "qwerty"]);
	assert_eq!(p.name, vec!["Test name", "check"]);
	assert_eq!(p.statement, vec!["Test statement", "3", "4"]);
	p.run();
}