mod kaleidoscope;
use kaleidoscope::lexer::{Lexer, Token};

use std::env;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader};


fn main() {
    let input = env::args().nth(1);

    let source: Box<dyn BufRead> = match input {
        None => Box::new(BufReader::new(io::stdin())),

        Some(filepath) => Box::new(
            BufReader::new(fs::File::open(filepath).unwrap())
        ),
    };

    let mut lexer = Lexer::new(source);

    loop {
        match lexer.gettok() {
            Token::TokEof => break,
            tok => { dbg!(tok); }
        }
    }
}
