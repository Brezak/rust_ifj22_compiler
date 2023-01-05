use std::{env, fs};

use chumsky::prelude::*;
use rust_ifj22_compiler::lexer::{lexer, print_lexer_errors};

fn main() {
    let file_name = env::args().nth(1).expect("Expected file argument");

    let src = fs::read_to_string(&file_name).expect("Failed to read file");

    let (tokens, errs) = lexer().parse_recovery(src.as_str());
    println!("{:?}", tokens);
    println!("{:?}", errs);

    print_lexer_errors(errs, &src, &file_name);
}
