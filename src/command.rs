#[derive(Debug, Clone, PartialEq)]
pub enum Command {
	Stack,
	Clear,
	Print,
	Help,
}

impl<'s> Command {
	pub fn parse(s: &str) -> Result<Command, &'static str> {
		match s {
			"s" => Ok(Command::Stack),
			"c" => Ok(Command::Clear),
			"p" => Ok(Command::Print),
			"h" => Ok(Command::Help),
			_ => Err("unknown command"),
		}
	}
}

pub fn is_command(s: &str) -> bool {
	match s {
		"s" | "c" | "p" | "h" => true,
		_ => false,
	}
}
