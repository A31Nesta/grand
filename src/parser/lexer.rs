use std::str::Chars;

use super::{token::Token, token_type::TokenType};

struct Lexer<'a> {
    source: String,
    source_chars: Chars<'a>,
    source_char_count: usize,
    lexeme_start: usize,
    char_reading: usize,
    line: usize,
    column: usize
}

impl<'a> Lexer<'a> {
    // tokenization
    fn tokenize(mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.is_eof() {
            self.lexeme_start = self.char_reading;
            let t = self.scan_token();
            if t.token_type != TokenType::Ignored {
                tokens.push(t);
            }
        }

        tokens
    }
    
    // big helper functions
    fn scan_token(&mut self) -> Token {
        let c = self.advance();

        let mut token = Token {
            token_type: TokenType::Ignored,
            content: String::new(),
            line: self.line,
            column: self.column,
        };

        match c {
            _ => {
                token.content = c.to_string();
            }
        }
        todo!()
    }

    // Small helper functions
    fn is_eof(&self) -> bool {
        return self.char_reading >= self.source_char_count
    }
    fn advance(&mut self) -> char {
        self.column += 1;
        self.char_reading += 1;
        self.source_chars.next().expect("Reading past EOF should be impossible")
    }

}

// API
pub fn tokenize(source: &str) -> Vec<Token> {
    let lexer = Lexer {
        source: source.to_string(),
        source_chars: source.chars(),
        source_char_count: source.chars().count(),
        lexeme_start: 0,
        char_reading: 0,
        line: 0,
        column: 0
    };

    lexer.tokenize()
}