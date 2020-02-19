use std::convert::TryFrom;

pub struct Expr {
	stack: Vec<Term>,
}

pub struct ParseError {
	kind: ParseErrorKind,
}

type ErrorPos = (usize, &'static str);

pub enum ParseErrorKind {
	InvalidOperator(ErrorPos),
	InvalidTerm(ErrorPos),
}

impl Expr {
	pub fn parse(s: &str) -> Result<Expr, ParseError> {
		let mut stack: Vec<Term> = Vec::new();
		for (i, t) in s.split_whitespace().enumerate() {
			let term = match t {
				"+" => Term::BinOp(BinOp::Add),
				"-" => Term::BinOp(BinOp::Sub),
				"*" => Term::BinOp(BinOp::Mul),
				"/" => Term::BinOp(BinOp::Div),
				"//" => Term::BinOp(BinOp::IDiv),
				"!" => Term::UnOp(UnOp::Fact),
				s => match s.parse::<f64>() {
					Ok(f) => Term::Number(f),
					Err(e) => {
						return Err(ParseError {
							kind: ParseErrorKind::InvalidTerm((i, "invalid number found")),
						})
					}
				},
			};

			stack.push(term);
		}

		Ok(Expr { stack })
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

pub enum BinOp {
	Add,
	Sub,
	Mul,
	Div,
	IDiv,
	Exp,
	Nrt,
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
