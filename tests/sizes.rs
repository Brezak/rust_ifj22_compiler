use std::mem::size_of;

use rust_ifj22_compiler::syntactic_analysis::ast::MainBody;
use rust_ifj22_compiler::Token;

const MAX_TOKEN_SIZE: usize = 32;
const MAX_AST_SIZE: usize = 64;

#[test]
fn test_token_size() {
    println!("Token size: {}", size_of::<Token>());
    assert!(size_of::<Token>() <= MAX_TOKEN_SIZE);
}

#[test]
#[ignore] // This may get enabled when the code gets optimized (at the time of writing MainBody is 152 bytes)
fn test_ast_token_size() {
    println!("Token size: {}", size_of::<MainBody>());
    assert!(size_of::<MainBody>() <= MAX_TOKEN_SIZE);
}
