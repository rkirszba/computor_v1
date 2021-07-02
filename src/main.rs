use std::error::Error;
use structopt::StructOpt;

use computor_v1::args;
use computor_v1::lexer::Lexer;
use computor_v1::parser::Parser;
use computor_v1::solver;

fn main() -> Result<(), Box<dyn Error>> {
    let input = args::UserInput::from_args();
    let mut lexer = Lexer::new();
    lexer.run(&input.equation)?;
    let lexems = lexer.get_lexems();
    let mut parser = Parser::new();
    parser.run(lexems)?;
    let degrees = parser.get_degrees();
    let mut solver = solver::choose_solver(degrees);
    solver.solve(degrees);
    solver.describe();
    Ok(())
}
