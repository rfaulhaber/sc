extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use sc::*;

fn main() {
	let mut stack = Stack::new();
	let mut rl = Editor::<()>::new();

	loop {
		let readline = rl.readline(">> ");
		match readline {
			Ok(line) => {
				if is_op(line.as_str()) {
					let res = stack.op(line.as_str());
					match res {
						Ok(val) => println!("{}", val),
						Err(e) => println!("{}", e),
					}
				} else {
					let parsed_float = line.parse::<f64>();

					match parsed_float {
						Ok(f) => stack.push(f),
						Err(_) => println!("could not parse number"),
					}
				}
			}
			Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
				break;
			}
			Err(err) => {
				println!("Error: {:?}", err);
				break;
			}
		}
	}
}
