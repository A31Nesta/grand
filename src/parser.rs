use std::io::Write;

mod token_type;
mod token;

mod lexer;

mod gex;

pub fn parse(source: &str) {
    let tokens = lexer::tokenize(source);

    for token in tokens {
        let token_color = match token.token_type {
            token_type::TokenType::Number => "\x1b[38;5;230m",
            token_type::TokenType::RangeCC => "\x1b[38;5;42m",
            token_type::TokenType::RangeOO => "\x1b[38;5;42m",
            token_type::TokenType::RangeCO => "\x1b[38;5;42m",
            token_type::TokenType::RangeOC => "\x1b[38;5;42m",
            token_type::TokenType::Comma => "\x1b[38;5;225m",
            token_type::TokenType::Not => "\x1b[38;5;192m",
            token_type::TokenType::LBrack => "\x1b[38;5;33m",
            token_type::TokenType::RBrack => "\x1b[38;5;33m",
            token_type::TokenType::LParen => "\x1b[38;5;141m",
            token_type::TokenType::RParen => "\x1b[38;5;141m",
            token_type::TokenType::Constraint => "\x1b[38;5;209m",
            token_type::TokenType::CMultOf => "\x1b[38;5;195m",
            token_type::TokenType::Ignored => "\x1b[0m",
        };

        print!("{}{}\x1b[0m", token_color, token.content);
    }
    std::io::stdout().flush().unwrap();

}