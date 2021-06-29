use std::{fmt, str};
use std::error::Error;


pub struct Lexer {
	lexems: Vec<Lexem>
}

impl Lexer {

	const TRANSITIONS: [[usize; 11]; 14] = [
		[ 0,  1,  2,  3,  4,  5,  6,  7, 13, 13, 12],
		[ 1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1],
		[ 2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2],
		[ 3,  3,  3,  3,  3,  3,  3,  3,  3,  3,  3],
		[ 4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4],
		[ 5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5],
		[ 6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6],
		[11, 11, 11, 11, 11, 11, 11,  7,  8, 11, 11],
		[13, 13, 13, 13, 13, 13, 13,  9, 13, 13, 13],
		[10, 10, 10, 10, 10, 10, 10,  9, 10, 10, 10],
		[10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
		[11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
		[12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12],
		[13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13]
	];
	
	const STATES: [State; 14] = [
		State::Initial,
		State::Final(Lexem::Plus { index: 0, len: 0 }),
		State::Final(Lexem::Minus { index: 0, len: 0 }),
		State::Final(Lexem::Mult { index: 0, len: 0 }),
		State::Final(Lexem::Power { index: 0, len: 0 }),
		State::Final(Lexem::Equal { index: 0, len: 0 }),
		State::Final(Lexem::X { index: 0, len: 0 }),
		State::Transitory,
		State::Transitory,
		State::Transitory,
		State::FinalStar(Lexem::Number { value: 0.0, index: 0, len: 0 }),
		State::FinalStar(Lexem::Number { value: 0.0, index: 0, len: 0 }),
		State::Final(Lexem::End { index: 0, len: 0 }),
		State::Error
	];

	pub fn new() -> Self {
		Lexer {
			lexems: Vec::new()
		}
	}
	
	fn add_lexem_nb(&mut self, equation: &str, start: usize, end: usize) -> Result<(), LexicalError>
	{
		let nb_str: &str = str::from_utf8(&(equation.as_bytes())[start..end]).unwrap();
		match nb_str.parse::<f64>(){
			Ok(nb) => {
				self.lexems.push(Lexem::Number{ value: nb, index: start, len: end - start });
				Ok(())
			},
			Err(_) => return Err(LexicalError::TooBigNumber(String::from(nb_str), start))
		}
	}

	fn add_lexem(&mut self, lexem_type: &Lexem, equation: &str, start: usize, end: usize) -> Result<(), LexicalError> {
		let _len: usize = end - start;
		match lexem_type {
			Lexem::Number { value:_, index:_, len:_ } => return self.add_lexem_nb(equation, start, end),
			Lexem::Plus { index:_, len:_ } => self.lexems.push(Lexem::Plus { index: start, len: _len}),
			Lexem::Minus { index:_, len:_ } => self.lexems.push(Lexem::Minus { index: start, len: _len}),
			Lexem::Mult { index:_, len:_ } => self.lexems.push(Lexem::Mult { index: start, len: _len}),
			Lexem::Equal { index:_, len:_ } => self.lexems.push(Lexem::Equal { index: start, len: _len}),
			Lexem::Power { index:_, len:_ } => self.lexems.push(Lexem::Power { index: start, len: _len}),
			Lexem::X { index:_, len:_ } => self.lexems.push(Lexem::X { index: start, len: _len}),
			Lexem::End { index:_, len:_ } => self.lexems.push(Lexem::End { index: start, len: _len})
		};
		Ok(())
	}

	fn get_state_machine_col(&self, c: char) -> usize {
		match c {
			val if val.is_ascii_whitespace() => 0,
			'+' => 1,
			'-' => 2,
			'*' => 3,
			'^' => 4,
			'=' => 5,
			'X' => 6,
			val if val.is_numeric() => 7,
			'.' => 8,
			val if val == 0x0 as char => 10,
			_ => 9
		}
	}


	// voir si possible de refacto la partie Final et FinalStar
	pub fn run(&mut self, equation: &str) -> Result<(), LexicalError> {
		let mut bytes: Vec<u8> = equation.as_bytes().to_vec();
		bytes.push(0x0);
		let len: usize = bytes.len();
		let mut cursor: usize = 0;
		let mut state: usize = 0;
		let mut lexem_start: usize = 0;
		
		self.lexems = Vec::new();
		while cursor < len {
			state = Lexer::TRANSITIONS[state][self.get_state_machine_col(bytes[cursor] as char)];
			match &Lexer::STATES[state] {
				State::Initial => lexem_start = cursor + 1,
				State::Transitory => (),
				State::Final(lexem_type) => {
					if let Err(e) = self.add_lexem(lexem_type, equation, lexem_start, cursor + 1) {
						return Err(e);
					};
					lexem_start = cursor + 1;
					state = 0;
				},
				State::FinalStar(lexem_type) => {
					if let Err(e) = self.add_lexem(lexem_type, equation, lexem_start, cursor) {
						return Err(e);
					};
					lexem_start = cursor;
					state = 0;
					cursor -= 1;
				},
				State::Error => return Err(LexicalError::UnexpectedCharacter(bytes[cursor] as char, cursor)),
			}
			cursor += 1;
		}
		Ok(())
	}

	pub fn get_lexems(&self) -> &Vec<Lexem> {
		&(self.lexems)
	}
}

pub enum LexicalError {
	UnexpectedCharacter(char, usize),
	TooBigNumber(String, usize)
}

impl fmt::Display for LexicalError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			LexicalError::UnexpectedCharacter(c, pos) =>
			write!(f, "Unexpected character '{}' found at index {}", c, pos),
			LexicalError::TooBigNumber(number, pos) =>
			write!(f, "'{}' (at index {}) is a too big number", number, pos)
        }
    }
}

impl<'a> fmt::Debug for LexicalError  {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "LexicalError: {}", self)
	}
}

impl Error for LexicalError {}



pub enum Lexem {
	Plus { index: usize, len: usize },
	Minus { index: usize, len: usize },
	Mult { index: usize, len: usize },
	Power { index: usize, len: usize },
	Equal { index: usize, len: usize },
	X { index: usize, len: usize },
	Number { value: f64, index: usize, len: usize },
	End { index: usize, len: usize }
}

pub enum State {
	Initial,
	Transitory,
	Final(Lexem),
	FinalStar(Lexem),
	Error,
}

impl fmt::Debug for Lexem {
	
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lexem::Plus { index:_, len:_ } => write!(f, "+"),
            Lexem::Minus { index:_, len:_ } => write!(f, "-"),
            Lexem::Mult { index:_, len:_ } => write!(f, "*"),
            Lexem::Power { index:_, len:_ } => write!(f, "^"),
            Lexem::Equal { index:_, len:_ } => write!(f, "="),
            Lexem::X { index:_, len:_ }=> write!(f, "X"),
            Lexem::Number { value, index:_, len:_ } => write!(f, "{}", value),
			Lexem::End { index:_, len:_ } => write!(f, "END")
        }
	}
}

