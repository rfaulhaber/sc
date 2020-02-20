extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use sc::expr::Expr;

fn main() {
	let mut rl = Editor::<()>::new();

	loop {
		let readline = rl.readline(">> ");
		match readline {
			Ok(line) => match &mut Expr::parse(line.as_str()) {
				Ok(expr) => match expr.evaluate() {
					Ok(val) => println!("{}", val),
					Err(e) => println!("error: {}", e),
				},
				Err(e) => println!("error: {}", e),
			},
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
