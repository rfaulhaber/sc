extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use sc::command::*;
use sc::expr::Expr;

const HELP_MESSAGE: &'static str = r#"sc help
	supported operations: + - * / 

	calculator commands:
	- h: prints this message
	- p: print top of stack
	- s: print stack

"#;

fn main() {
    let mut rl = Editor::<()>::new();

    let mut stack: Vec<f64> = Vec::new();

    loop {
        let readline = rl.readline("");
        match readline {
            Ok(line) if is_command(line.as_str()) => match Command::parse(line.as_str()) {
                Ok(c) => match c {
                    Command::Stack => {
                        print!("[ ");
                        for f in &stack {
                            print!("{} ", f);
                        }

                        print!(" ]");
                    }
                    Command::Print => println!(
                        "{}",
                        stack
                            .last()
                            .map(|f| f.to_string())
                            .unwrap_or_else(|| String::from("sc: stack empty"))
                    ),
                    Command::Clear => {
                        stack.clear();
                        println!("stack cleared");
                    }
                    Command::Help => {
						println!("{}", HELP_MESSAGE);
					}
                },
                Err(s) => println!("{}", s),
            },
            Ok(line) => match &mut Expr::parse(line.as_str()) {
                Ok(e) => match e.evaluate(&mut stack) {
                    Ok(val) => {
                        stack.push(val);
                    }
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
