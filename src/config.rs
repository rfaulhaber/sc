pub struct Config {
	pub mode: Mode,
}

impl Default for Config {
	fn default() -> Config {
		Config { mode: Mode::Basic }
	}
}

pub enum Mode {
	Basic,
	Scientific,
	Programmer,
}
