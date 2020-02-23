use std::fmt;

#[derive(Debug, Default)]
pub struct Expr {
	stack: Vec<Term>,
}

#[derive(Debug)]
pub struct ParseError {
	kind: ParseErrorKind,
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.kind {
			ParseErrorKind::InvalidTerm(s) | ParseErrorKind::InvalidOperator(s) => {
				write!(f, "{}", s)
			}
		}
	}
}

#[derive(Debug)]
pub enum ParseErrorKind {
	InvalidOperator(String),
	InvalidTerm(String),
}

impl Expr {
	pub fn parse(s: &str) -> Result<Expr, ParseError> {
		let stack = make_stack(s)?;
		Ok(Expr { stack })
	}

	pub fn evaluate(&mut self, stack: &mut Vec<f64>) -> Result<f64, &'static str> {
		if self.stack.is_empty() {
			return Err("stack empty");
		}

		while !self.stack.is_empty() {
			let t = self.stack.pop();

			match t {
				Some(Term::Number(n)) => stack.push(n),
				Some(Term::BinOp(op)) => {
					let right_op = stack.pop();
					let left_op = stack.pop();
					let op = get_bin_op(op);

					match (left_op, right_op) {
						(Some(l), Some(r)) => stack.push(op(l, r)),
						(None, Some(r)) => stack.push(r),
						_ => return Err("not enough items on stack"),
					}
				}
				Some(Term::UnOp(op)) => unimplemented!(),
				None => unimplemented!(),
			}
		}

		if stack.is_empty() {
			Err("empty stack")
		} else {
			let result = stack.pop().unwrap();
			self.stack.push(Term::Number(result));
			Ok(result)
		}
	}

	pub fn push_expr(&mut self, mut e: Expr) {
		e.stack.extend(self.stack.clone());
		self.stack = e.stack;
	}
}

fn parse_term(s: &str) -> Result<Term, ParseError> {
	match s {
		"+" => Ok(Term::BinOp(BinOp::Add)),
		"-" => Ok(Term::BinOp(BinOp::Sub)),
		"*" => Ok(Term::BinOp(BinOp::Mul)),
		"/" => Ok(Term::BinOp(BinOp::Div)),
		"//" => Ok(Term::BinOp(BinOp::IDiv)),
		"!" => Ok(Term::UnOp(UnOp::Fact)),
		s => match s.parse::<f64>() {
			Ok(f) => Ok(Term::Number(f)),
			Err(_) => Err(ParseError {
				kind: ParseErrorKind::InvalidTerm(format!("invalid token found: {}", s)),
			}),
		},
	}
}

fn make_stack(s: &str) -> Result<Vec<Term>, ParseError> {
	let mut stack = Vec::new();

	for t in s.split_whitespace() {
		let term = parse_term(t)?;
		stack.push(term);
	}

	stack.reverse();

	Ok(stack)
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
	Add,
	Sub,
	Mul,
	Div,
	IDiv,
}

#[derive(Debug, Clone, PartialEq)]
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
		BinOp::IDiv => |l, r| (l as i64 / r as i64) as f64,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn expr_parses() {
		let input = "1 2 +";
		let expected = vec![
			Term::BinOp(BinOp::Add),
			Term::Number(2.0),
			Term::Number(1.0),
		];

		let expr = Expr::parse(input).unwrap();

		assert_eq!(expr.stack, expected);
	}

	#[test]
	fn expr_evaluates() {
		let input = "1 2 + +";

		let result = Expr::parse(input).unwrap().evaluate(&mut vec![3.0]);

		assert_eq!(Ok(6.0), result);
	}

	#[test]
	fn expr_push_evaluates() {
		let mut expr = Expr {
			stack: vec![
				Term::BinOp(BinOp::Add),
				Term::Number(2.0),
				Term::Number(1.0),
			],
		};

		let expr_right = Expr {
			stack: vec![Term::BinOp(BinOp::Div), Term::Number(3.0)],
		};

		expr.push_expr(expr_right);

		let expected_stack = vec![
			Term::BinOp(BinOp::Div),
			Term::Number(3.0),
			Term::BinOp(BinOp::Add),
			Term::Number(2.0),
			Term::Number(1.0),
		];

		assert_eq!(expr.stack, expected_stack);
	}

	#[test]
	fn pushed_expr_evaluates() {
		let mut expr = Expr {
			stack: vec![
				Term::BinOp(BinOp::Add),
				Term::Number(2.0),
				Term::Number(1.0),
			],
		};

		let expr_right = Expr {
			stack: vec![Term::BinOp(BinOp::Div), Term::Number(3.0)],
		};

		expr.push_expr(expr_right);

		let result = expr.evaluate(&mut vec![]);

		assert_eq!(Ok(1.0), result);
	}
}
