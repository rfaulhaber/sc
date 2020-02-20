pub struct Expr {
	stack: Vec<Term>,
}

pub struct ParseError {
	kind: ParseErrorKind,
}

pub enum ParseErrorKind {
	InvalidOperator(String),
	InvalidTerm(String),
}

impl Expr {
	pub fn parse(s: &str) -> Result<Expr, ParseError> {
		let mut stack: Vec<Term> = Vec::new();

		for t in s.split_whitespace() {
			let term = match t {
				"+" => Term::BinOp(BinOp::Add),
				"-" => Term::BinOp(BinOp::Sub),
				"*" => Term::BinOp(BinOp::Mul),
				"/" => Term::BinOp(BinOp::Div),
				// "//" => Term::BinOp(BinOp::IDiv),
				"!" => Term::UnOp(UnOp::Fact),
				s => match s.parse::<f64>() {
					Ok(f) => Term::Number(f),
					Err(_) => {
						return Err(ParseError {
							kind: ParseErrorKind::InvalidTerm(format!(
								"invalid token found: {}",
								t
							)),
						})
					}
				},
			};

			stack.push(term);
		}

		stack.reverse();

		Ok(Expr { stack })
	}

	pub fn evaluate(&mut self) -> Result<f64, &'static str> {
		if self.stack.is_empty() {
			return Err("stack empty");
		}

		let mut local_stack = Vec::new();

		while !self.stack.is_empty() {
			let t = self.stack.pop();

			match t {
				Some(Term::Number(n)) => local_stack.push(n),
				Some(Term::BinOp(op)) => {
					let right_op = local_stack.pop();
					let left_op = local_stack.pop();
					let op = get_bin_op(op);

					match (left_op, right_op) {
						(Some(l), Some(r)) => local_stack.push(op(l, r)),
						(None, Some(r)) => local_stack.push(r),
						_ => return Err("not enough items on stack"),
					}
				}
				Some(Term::UnOp(op)) => unimplemented!(),
				None => unimplemented!(),
			}
		}

		if local_stack.is_empty() {
			Err("empty stack")
		} else {
			Ok(local_stack.pop().unwrap())
		}
	}
}

pub enum Term {
	Number(f64),
	BinOp(BinOp),
	UnOp(UnOp),
}

impl From<BinOp> for Term {
	fn from(op: BinOp) -> Term {
		Term::BinOp(op)
	}
}

impl From<UnOp> for Term {
	fn from(op: UnOp) -> Term {
		Term::UnOp(op)
	}
}

impl Term {
	fn is_op(&self) -> bool {
		match self {
			Term::BinOp(_) | Term::UnOp(_) => true,
			_ => false,
		}
	}

	fn is_number(&self) -> bool {
		match self {
			Term::Number(_) => true,
			_ => false,
		}
	}
}

pub enum BinOp {
	Add,
	Sub,
	Mul,
	Div,
}

pub enum UnOp {
	Fact,
	Sin,
	Cos,
	Tan,
	Sqrt,
	Ln,
	Log,
}

fn is_number(s: &str) -> bool {
	s.parse::<f64>().is_ok()
}

fn get_bin_op(op: BinOp) -> fn(f64, f64) -> f64 {
	match op {
		BinOp::Add => |l, r| l + r,
		BinOp::Sub => |l, r| l - r,
		BinOp::Mul => |l, r| l * r,
		BinOp::Div => |l, r| l / r,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn evaluate_short_returns_result() {
		let input = "1 2 +";
		let expected = 3.0;

		let expr = Expr::parse(input);
	}
}
