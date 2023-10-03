mod frontend;

use std::error::Error;
use std::{fs, env};

use frontend::lexer::*;
// use frontend::ast_parser::*;

fn main() -> Result<(), Box<dyn Error>>g
    env::set_var("RUST_BACKTRACE", "1");

    let mut source = fs::read_to_string("./src/test_files/test1.lp")?;
    source.push('\0');

    let token_stream = Lexer::new(&source).lex()?;
    println!("{:?}", token_stream);

    Ok(())
}
