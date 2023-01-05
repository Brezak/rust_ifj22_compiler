use std::mem::size_of;
use RustIFJ22Parser::Token;

const MAX_TOKEN_SIZE: usize = 32;
const MAX_AST_SIZE: usize = 64;

#[test]
pub fn test_token_size() {
    println!("Token size: {}", size_of::<Token>());
    assert!(size_of::<Token>() <= MAX_TOKEN_SIZE);
}
