mod token_type;
mod token;

mod lexer;

pub fn parse(source: &str) {
    let _tokens = lexer::tokenize(source);
}