use std::fs::File;
use std::process::{Command, Stdio};
use std::io::prelude::*;
use std::io::BufReader;

const PROBLEMS_DIR: &'static str = "problems";
const INPUT_DIR: &'static str = "input";
const OUTPUT_DIR: &'static str = "output";

pub struct Problem {
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
	pub fn new(str_id: String) -> Problem {
		let mut statement = vec![];
		let mut name = vec![];
		let mut authors = vec![];
		match File::open(format!("{}/{}/info.txt", PROBLEMS_DIR, str_id)) {
			Ok(file) => {
				let buf = BufReader::new(&file);
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
			},
			Err(err) => println!("Unable to load problem \"{}\" info: {}", str_id, err),
		};
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
			self.run_test(1);	
		} else {
			let s = String::from_utf8_lossy(&builder.stderr);
			panic!("rustc failed to build problem {}:\n{}", self.str_id, s);
		}
	}

	fn run_test(&self, test_no: u32) {
		let mut input = File::open(format!("{}/{}/{}/{}.txt", PROBLEMS_DIR, self.str_id, INPUT_DIR, test_no))
			.unwrap_or_else(|e| { panic!("failed to load test \"{}\": {}", test_no, e) });
		let mut s = String::new(); // passing throw String coz as_slice still unstable
		input.read_to_string(&mut s)
			.unwrap_or_else(|e| { panic!("couldn't read test \"{}\" file: {}", test_no, e) });

		let problem = Command::new(format!("{}/{}/{}", PROBLEMS_DIR, self.str_id, self.str_id))
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn()
			.unwrap_or_else(|e| { panic!("failed to execute test \"{}\": {}", test_no, e) });
		
		problem.stdin.unwrap().write_all(s.as_bytes())
			.unwrap_or_else(|e| { panic!("couldn't write stdin test \"{}\": {}", test_no, e) });
		{
			let mut s = String::new(); // passing throw String coz as_slice still unstable
			problem.stdout.unwrap().read_to_string(&mut s)
				.unwrap_or_else(|e| { panic!("couldn't read stdout test \"{}\": {}", test_no, e) });

			let mut output = File::create(format!("{}/{}/{}/{}.txt", PROBLEMS_DIR, self.str_id, OUTPUT_DIR, test_no))
				.unwrap_or_else(|e| { panic!("failed to save test \"{}\" output: {}", test_no, e) }); 

			output.write_all(s.as_bytes())
				.unwrap_or_else(|e| { panic!("couldn't write test \"{}\" answer: {}", test_no, e) });
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