use std::fs::File;
use std::process::{Command, Stdio};
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;
use std::fmt;

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

struct ProblemError {
	sys_part: String,
	reason: &'static str,
	problem_strid: String,
	test_no: i32,
}

impl Error for ProblemError {
	fn description(&self) -> &str {
        self.reason
    }
}

impl fmt::Display for ProblemError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.test_no >= 0 {
			write!(f, "{} on problem {} at test {}: {}", self.reason, self.problem_strid, 
				self.test_no, self.sys_part)
		}
		else {
			write!(f, "{} on problem {}: {}", self.reason, self.problem_strid, 
				self.sys_part)
		}
	}
}

impl fmt::Debug for ProblemError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.test_no >= 0 {
			write!(f, "{} on problem {} at test {}: {}", self.reason, self.problem_strid, 
				self.test_no, self.sys_part)
		}
		else {
			write!(f, "{} on problem {}: {}", self.reason, self.problem_strid, 
				self.sys_part)
		}
	}
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
			Err(err) => panic!("Unable to load problem \"{}\" info: {}", str_id, err),
		};	
		Problem {
			str_id: str_id,
			statement: statement,
			name: name,
			authors: authors,
		}
	}
	pub fn run(&self) -> Result<(), ProblemError> {
		let builder = match Command::new("rustc")
			.arg(format!("{}/{}/{}.rs", PROBLEMS_DIR, self.str_id, self.str_id))
			.arg("--out-dir")
			.arg(format!("{}/{}", PROBLEMS_DIR, self.str_id))
			.output() {
				Ok(cmd) => cmd,
				Err(e) => return self.make_error("failed to execute rustc", e.to_string(), -1),
			};

		if !builder.status.success() {
			let s = String::from_utf8_lossy(&builder.stderr);
			return self.make_error("rustc failed to build problem", s.into_owned(), -1)	
		}
		return self.run_test(1);
	}

	fn run_test(&self, test_no: i32) -> Result<(), ProblemError> {
		let mut input = match File::open(format!("{}/{}/{}/{}.txt", 
			PROBLEMS_DIR, self.str_id, INPUT_DIR, test_no)) {
			Ok(file) => file,
			Err(e) => return self.make_error("failed to load test", e.to_string(), test_no),
		};

		let mut s = String::new(); // passing throw String coz as_slice still unstable
		let result = input.read_to_string(&mut s);
		if result.is_err() {
			return self.make_error("couldn't read test", result.err().unwrap().to_string(), test_no);
		}

		let problem = match Command::new(format!("{}/{}/{}", PROBLEMS_DIR, self.str_id, self.str_id))
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn() {
				Ok(cmd) => cmd,
				Err(e) => return self.make_error("failed to execute test", e.to_string(), test_no),	
			};
		
		let result = problem.stdin.unwrap().write_all(s.as_bytes());
		if result.is_err() {
			return self.make_error("couldn't send stdin to test", result.err().unwrap().to_string(), test_no);
		}

		let mut s = String::new(); // passing throw String coz as_slice still unstable
		let result = problem.stdout.unwrap().read_to_string(&mut s);
		if result.is_err() {
			return self.make_error("couldn't read stdout from test", result.err().unwrap().to_string(), test_no);
		}

		let mut output = match File::create(format!("{}/{}/{}/{}.txt", 
			PROBLEMS_DIR, self.str_id, OUTPUT_DIR, test_no)) {
			Ok(file) => file,
			Err(e) => return self.make_error("failed to create test output file", e.to_string(), test_no),
		};

		let result = output.write_all(s.as_bytes());
		if result.is_err() {
			return self.make_error("couldn't write test answer", result.err().unwrap().to_string(), test_no);
		}
		Ok(())
	}

	fn make_error(&self, reason: &'static str, sys_part: String, test_no: i32) -> Result<(), ProblemError> {
		Err(ProblemError { 
			reason:reason, 
			sys_part: sys_part, 
			problem_strid: self.str_id.clone(), 
			test_no: test_no
		})
	}
}

#[test]
fn read_test() {
	let p = Problem::new("test".to_string());
	assert_eq!(p.str_id, "test");
	assert_eq!(p.authors, vec!["Testhor", "qwerty"]);
	assert_eq!(p.name, vec!["Test name", "check"]);
	assert_eq!(p.statement, vec!["Test statement", "3", "4"]);
	let result = p.run();
	if result.is_err() {
		panic!(result.err().unwrap().to_string());
	} 
}