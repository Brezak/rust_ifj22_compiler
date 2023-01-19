use chumsky::prelude::*;

// TODO: Fix recursive multiline comments
fn block_comment() -> impl Parser<char, (), Error = Simple<char>> + Clone {
    let start_mark = just("/*").padded();
    let end_mark = just("*/").padded();

    start_mark.ignore_then(take_until(end_mark)).ignored()
}

pub fn comment() -> impl Parser<char, (), Error = Simple<char>> + Clone {
    just("//")
        .then(take_until(just("\n")))
        .ignored()
        .or(block_comment())
}
