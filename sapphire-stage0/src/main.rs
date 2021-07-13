pub mod ast;
pub mod macros;
pub mod parser;
pub mod source_location;
pub mod symbol_repr;
pub mod token;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Command {
    Parse {
        #[structopt(short = "f")]
        file_name: String,
    },
}

fn main() {
    println!("Hello, world!");
}
