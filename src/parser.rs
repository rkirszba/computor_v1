use std::fmt;
use std::error::Error;
use std::collections::HashMap;

use crate::lexer::Lexem;

pub struct Parser {
	degrees: HashMap<u32, f64>
}












pub enum ParseError {
	UnexpectedToken(Lexem),
	NotIntegerDegree(Lexem)
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ParseError::UnexpectedToken(lexem) =>
			write!(f, "Unexpected token '{:?}' found at index {}", lexem, lexem.get_index()),
			ParseError::NotIntegerDegree(lexem) =>
			write!(f, "{}, found at index {}, is not an integer degree", lexem.get_value(), lexem.get_index()),
		}
	} 
}

impl<'a> fmt::Debug for ParseError  {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "ParseError: {}", self)
	}
}

impl Error for ParseError {}