use std::fmt;
use std::error::Error;
use std::collections::HashMap;

use crate::lexer::Lexem;

pub struct Parser {
	degrees: HashMap<u32, f64>

}

impl <'a> Parser {

	pub fn new() -> Self {
		let mut degrees: HashMap<u32, f64> = HashMap::new();
		degrees.insert(0, 0.0);
		Parser {
			degrees
		}
	}

	fn update_hashmap(&mut self, term: &Term) {
		let val: &mut f64 = self.degrees.entry(term.degree).or_insert(0.0);
		*val += term.coeff;
		if *val == 0.0 && term.degree != 0 {
			self.degrees.remove(&term.degree);
		}
	} 

	fn get_next_lexem(&self, lexems: &'a Vec<Lexem>, cursor: &mut usize) -> Result<&'a Lexem, ParseError>
	{
		if *cursor == lexems.len() {
			return Err(ParseError::NoTokenProvided());
		}
		let lexem: &Lexem = &lexems[*cursor];
		*cursor += 1;
		Ok(lexem)
	}

	fn check_expected_terminal_symbol(&self, lexems: &Vec<Lexem>, cursor: &mut usize, target: &Lexem) -> Result<(), ParseError>
	{
		let lexem = self.get_next_lexem(lexems, cursor)?;
		match *lexem == *target {
			true => Ok(()),
			false => Err(ParseError::UnexpectedToken(*lexem))
		}
	}

	fn degree(&mut self, lexems: &Vec<Lexem>, cursor: &mut usize) -> Result<u32, ParseError> {
		match self.get_next_lexem(lexems, cursor)? {
			lexem if *lexem == Lexem::Power { index: 0, len: 0 } => {
				match self.get_next_lexem(lexems, cursor)? {
					lexem_2 if *lexem_2 == Lexem::Number { value: 0.0, index: 0, len: 0 } => {
						let degree = lexem_2.get_value();
						if degree.fract() != 0.0 || !(degree >= u32::MIN as f64 && degree <= u32::MAX as f64) {
							return Err(ParseError::NotUIntegerDegree(*lexem_2));
						}
						Ok(degree as u32)
					},
					lexem_2 => Err(ParseError::UnexpectedToken(*lexem_2))
				}
			},
			_ => { *cursor -= 1; Ok(1) }
		}
	}

	fn term_end(&mut self, lexems: &Vec<Lexem>, cursor: &mut usize) -> Result<u32, ParseError> {
		match self.get_next_lexem(lexems, cursor)? {
			lexem if *lexem == Lexem::Mult { index: 0, len: 0 } => {
				self.check_expected_terminal_symbol(lexems, cursor, &Lexem::X { index: 0, len: 0 })?;
				self.degree(lexems, cursor)
			}
			lexem if *lexem == Lexem::X { index: 0, len: 0 } => self.degree(lexems, cursor),
			_ => { *cursor -= 1; Ok(0) }
		}
	}

	fn term(&mut self, lexems: &Vec<Lexem>, cursor: &mut usize) -> Result<Term, ParseError> {
		match self.get_next_lexem(lexems, cursor)? {
			lexem if *lexem == Lexem::Number { value: 0.0, index: 0, len: 0 } =>
			Ok(Term {
				coeff: lexem.get_value(),
				degree: self.term_end(lexems, cursor)?
			}),
			lexem if *lexem == Lexem::X { index: 0, len: 0 } =>
			Ok (Term {
				coeff: 1.0,
				degree: self.degree(lexems, cursor)?
			}),
			lexem => Err(ParseError::UnexpectedToken(*lexem))
		}
	}

	fn expression_end(&mut self, lexems: &Vec<Lexem>, cursor: &mut usize, member: Member) -> Result<(), ParseError> {
		let mut sign:i32 = match member {
			Member::Left => 1,
			Member::Right => -1
		};
		match self.get_next_lexem(lexems, cursor)? {
			lexem if *lexem == Lexem::Plus { index: 0, len: 0 } => (),
			lexem if *lexem == Lexem::Minus { index: 0, len: 0 } => sign *= -1,
			_ => { *cursor -= 1; return Ok(()) }
		};
		let mut term = self.term(lexems, cursor)?;
		term.coeff *= sign as f64;
		self.update_hashmap(&term);
		self.expression_end(lexems, cursor, member)
	}

	fn expression(&mut self, lexems: &Vec<Lexem>, cursor: &mut usize, member: Member) -> Result<(), ParseError> {
		let mut sign:i32 = match member {
			Member::Left => 1,
			Member::Right => -1
		};
		match self.get_next_lexem(lexems, cursor)? {
			lexem if *lexem == Lexem::Plus { index: 0, len: 0 } => (),
			lexem if *lexem == Lexem::Minus { index: 0, len: 0 } => sign *= -1,
			_ =>  *cursor -= 1
		};
		let mut term = self.term(lexems, cursor)?;
		term.coeff *= sign as f64;
		self.update_hashmap(&term);
		self.expression_end(lexems, cursor, member)
	}

	fn equation(&mut self, lexems: &Vec<Lexem>) -> Result<(), ParseError> {
		let mut cursor: usize = 0;
		self.expression(lexems, &mut cursor, Member::Left)?;
		self.check_expected_terminal_symbol(lexems, &mut cursor, &Lexem::Equal { index: 0, len: 0 })?;
		self.expression(lexems, &mut cursor, Member::Right)?;
		self.check_expected_terminal_symbol(lexems, &mut cursor, &Lexem::End { index: 0, len: 0 })
	}	
	
	pub fn run(&mut self, lexems: &Vec<Lexem>) -> Result<(), ParseError> {
		self.degrees = HashMap::new();
		self.equation(lexems)	
	}

	pub fn get_degrees(&self) -> &HashMap<u32, f64> {
		&self.degrees
	}
}


#[derive(Copy, Clone)]
enum Member {
	Left,
	Right
}

struct Term {
	coeff: f64,
	degree: u32
}

pub enum ParseError {
	UnexpectedToken(Lexem),
	NotUIntegerDegree(Lexem),
	NoTokenProvided()
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ParseError::UnexpectedToken(lexem) =>
			write!(f, "Unexpected token '{:?}' found at index {}", lexem, lexem.get_index()),
			ParseError::NotUIntegerDegree(lexem) =>
			write!(f, "{}, found at index {}, is not an unsigned integer degree", lexem.get_value(), lexem.get_index()),
			ParseError::NoTokenProvided() =>
			write!(f, "No token was provided")
		}
	} 
}

impl<'a> fmt::Debug for ParseError  {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "ParseError: {}", self)
	}
}

impl Error for ParseError {}