use crate::err_ctx;
use crate::state::State;
use crate::word::Word;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, Write};

const FILE_NAME: &str = "hanzi.tsv";

pub fn read_file() -> Result<(State, Vec<Word>), String> {
	let file = match open_file(OpenOptions::new().read(true)) {
		Ok(x) => x,
		Err(e) => return Err(e)
	};

	let reader = BufReader::new(file);

	let mut lines = reader.lines();
	let first_line = match lines.next() {
		Some(x) => x,
		None => return Err(String::from("File is empty"))
	};

	let state = match first_line {
		Ok(x) => State::deserialize(&x),
		Err(e) => return Err(err_ctx!(e, "Failed to read line 1"))
	};

	let mut line_num = 2;
	let mut words: Vec<Word> = vec![];
	for line in lines {
		match line {
			Ok(x) => words.push(Word::deserialize(&x)),
			Err(e) => return Err(err_ctx!(e, format!("Failed to read line {}", line_num).as_str()))
		}
		line_num += 1;
	}

	return Ok((state, words));
}

pub fn write_file(state: &State, words: &Vec<Word>) -> Option<String> {
	let mut file = match open_file(OpenOptions::new().write(true).truncate(true)) {
		Ok(x) => x,
		Err(e) => return Some(e)
	};

	if let Err(e) = writeln!(file, "{}", state.serialize()) {
		return Some(err_ctx!(e, "Failed to write state to file"));
	}

	for word in words {
		if let Err(e) = writeln!(file, "{}", word.serialize()) {
			return Some(err_ctx!(e, "Failed to write word to file"));
		}
	}

	return None;
}

fn open_file(options: &OpenOptions) -> Result<File, String> {
	let mut exe_path = match std::env::current_exe() {
		Ok(x) => x,
		Err(e) => return Err(err_ctx!(e, "Failed to find the file"))
	};

	exe_path.pop();
	exe_path.push(FILE_NAME);
	return match options.open(&exe_path) {
		Ok(x) => Ok(x),
		Err(e) => Err(err_ctx!(e, "Failed to the open file"))
	};
}

fn err(ctx: ErrorContext) -> String {
	println!("{:?}", ctx.error);
	println!("at {} {}:{}", ctx.file, ctx.line, ctx.column);
	eprintln!("{:?}", ctx.error);
	eprintln!("at {} {}:{}", ctx.file, ctx.line, ctx.column);
	return ctx.to_display;
}

struct ErrorContext {
	file: String,
	line: u32,
	column: u32,
	error: Error,
	to_display: String
}

#[macro_export]
macro_rules! err_ctx {
	( $error: expr, $msg: expr) => {
		err(ErrorContext {
			file: file!().to_string(),
			line: line!(),
			column: column!(),
			error: $error,
			to_display: $msg.to_string(),
		})
	};
}
