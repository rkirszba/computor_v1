use std::error::Error;
use structopt::StructOpt;

use computor_v1::args;
use computor_v1::lexer::{Lexer};

fn main() -> Result<(), Box<dyn Error>> {
    
    let input = args::UserInput::from_args();
    let mut lexer = Lexer::new();
    lexer.run(&input.equation)?;
    let lexems = lexer.get_lexems();
    for lexem in lexems {
        println!("{:?} ", lexem);
    }
    Ok(())
}
