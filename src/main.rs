use std::error::Error;
use structopt::StructOpt;

use computor_v1::args;
use computor_v1::lexer::Lexer;
use computor_v1::parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    
    let input = args::UserInput::from_args();
    let mut lexer = Lexer::new();
    lexer.run(&input.equation)?;
    let lexems = lexer.get_lexems();
    // for lexem in lexems {
    //     println!("{:?} ", lexem);
    // }
    let mut parser = Parser::new();
    parser.run(lexems)?;
    let degrees = parser.get_degrees();
    for (degree, coeff) in degrees {
        println!("Degree: {} -- Coeff: {}", degree, coeff);
    }
    Ok(())
}
