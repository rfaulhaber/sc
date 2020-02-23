#[derive(Debug, Clone, PartialEq)]
pub enum Command {
	Stack,
	Clear,
}

impl<'s> Command {
	pub fn parse(s: &str) -> Result<Command, &'static str> {
		match s {
			":stack" => Ok(Command::Stack),
			":clear" => Ok(Command::Clear),
			_ => Err("unknown command"),
		}
	}
}
