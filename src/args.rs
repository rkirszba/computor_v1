use structopt::StructOpt;

#[derive(StructOpt)]
pub struct UserInput {
    pub equation: String,
}