pub struct Stack {
	stack: Vec<f64>,
}

pub type StackResult<T> = Result<T, &'static str>;

pub enum Op {
	Add,
	Sub,
	Mul,
	Div,
	IDiv,
	Exp,
}

impl Stack {
	pub fn new() -> Stack {
		Stack { stack: Vec::new() }
	}

	pub fn push(&mut self, val: f64) {
		self.stack.push(val)
	}

	pub fn op(&mut self, op: &str) -> StackResult<f64> {
		let op_fn = get_op_fn(op);

		match op_fn {
			Some(f) => self.bin_op(f),
			_ => Err("unknown operator"),
		}
	}

	fn bin_op<F>(&mut self, f: F) -> StackResult<f64>
	where
		F: FnOnce(f64, f64) -> f64,
	{
		let lop = self.stack.pop();
		let rop = self.stack.pop();

		match (rop, lop) {
			(Some(l), Some(r)) => {
				let result = f(l, r);
				self.stack.push(result);
				Ok(result)
			}
			_ => Err("not enough items on the stack"),
		}
	}
}

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

pub fn is_op(s: &str) -> bool {
	match s {
		"+" | "-" | "*" | "/" => true,
		_ => false,
	}
}

type OpFn = fn(f64, f64) -> f64;

fn get_op_fn(s: &str) -> Option<OpFn> {
	match s {
		"+" => Some(|l, r| l + r),
		"-" => Some(|l, r| l - r),
		"*" => Some(|l, r| l * r),
		"/" => Some(|l, r| l / r),
		_ => None,
	}
}
