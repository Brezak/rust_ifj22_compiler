mod common;

use chumsky::prelude::Parser;
use rust_ifj22_compiler::lexer;
use rust_ifj22_compiler::lexer::print_lexer_errors;

#[test]
fn empty_file() {
    let file_name = "empty.txt";
    let file = common::load_test_code(file_name);

    let lex = lexer();

    let result = lex.parse(file.as_str());
    if let Err(errs) = result {
        print_lexer_errors(errs, &file, file_name);
        panic!("Lexer failed to lex an empty file!");
    }
}

#[test]
fn unexpected_char() {
    let group_name = "unexpected_chars";
    let files = common::load_test_group(group_name);

    let lex = lexer();

    for (file, src) in files {
        if let Ok(tree) = lex.parse(src) {
            println!("{:?}", tree);
            panic!("Lexer failed to spot the mistake in file {file}");
        }
    }
}
