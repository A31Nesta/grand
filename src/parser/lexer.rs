use std::{io::Write, str::Chars};

use super::{token::Token, token_type::TokenType};

struct Lexer<'a> {
    source_chars: Chars<'a>,
    source_char_count: usize,
    lexeme_start: usize,
    char_reading: usize,
    char_reading_value: char,
    line: usize,
    column: usize,
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
            content: c.to_string(), // Default
            line: self.line,
            column: self.column,
        };

        match c {
            // Single-character tokens
            '!' => token.token_type = TokenType::Not,
            '[' => token.token_type = TokenType::LBrack,
            ']' => token.token_type = TokenType::RBrack,
            '(' => token.token_type = TokenType::LParen,
            ')' => token.token_type = TokenType::RParen,
            '|' => token.token_type = TokenType::Constraint,
            // Constraint Types
            '*' => token.token_type = TokenType::CMultOf,

            // Ranges
            ',' => {
                let next = self.peek();
                if next == '.' || next == ',' {
                    // This is a range
                    let token_info = self.read_range();
                    token.token_type = token_info.0;
                    token.content = token_info.1;
                } else {
                    // This is just a separator (Comma)
                    token.token_type = TokenType::Comma
                }
            }
            '.' => {
                let next = self.peek();
                if next == '.' || next == ',' {
                    // This is a range
                    let token_info = self.read_range();
                    token.token_type = token_info.0;
                    token.content = token_info.1;
                }
                // If this is not a range we just ignore the token.
                // Grand Expressions are just chill like that.
            }

            // New Lines
            '\n' => {
                self.line += 1;
                self.column = 0;
            }

            // Any letter, space, etc. is ignored
            // We only have to check for numbers
            _ => {
                if self.is_digit(c) || c == '-' || c == '+' {
                    let token_info = self.read_numeric();
                    token.token_type = token_info.0;
                    token.content = token_info.1;
                }
                // If this is not a number we ignore the token
            }
        }
        
        token
    }

    fn read_range(&mut self) -> (TokenType, String) {
        let c1 = self.char_reading_value;
        let c2 = self.advance();
        let token_content = format!("{c1}{c2}");
        match (c1, c2) {
            ('.', '.') => (TokenType::RangeCC, token_content),
            ('.', ',') => (TokenType::RangeCO, token_content),
            (',', '.') => (TokenType::RangeOC, token_content),
            (',', ',') => (TokenType::RangeOO, token_content),
            _ => {
                panic!("Invalid range characters. This is controlled by the scan_token function and should never happen")
            }
        }
    }

    fn read_numeric(&mut self) -> (TokenType, String) {
        let mut number = self.char_reading_value.to_string();
        // add digits to number
        while self.is_digit(self.peek()) {
            number.push(self.advance());
        }
        // If we reached a dot and there are more numbers afterwards we take it and continue reading
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            number.push(self.advance());
            // Continue reading
            while self.is_digit(self.peek()) {
                number.push(self.advance());
            }
        }

        (TokenType::Number, number)
    }

    // Small helper functions
    fn is_eof(&self) -> bool {
        return self.char_reading >= self.source_char_count
    }
    fn is_digit(&self, c: char) -> bool {
        c.is_digit(10)
    }
    fn peek(&self) -> char {
        self.source_chars
            .clone()
            .peekable()
            .peek()
            .unwrap_or(&'\0')
            .to_owned()
    }
    fn peek_next(&self) -> char {
        let mut clone_chars = self.source_chars.clone();
        // Advance 1
        clone_chars.next().unwrap_or_else(|| {
            return '\0';
        });
        // Advance a second time
        clone_chars.next().unwrap_or('\0')
    }
    fn advance(&mut self) -> char {
        self.column += 1;
        self.char_reading += 1;
        self.char_reading_value = self.source_chars.next().unwrap_or('\n');
        self.char_reading_value
    }

}

// API
pub fn tokenize(source: &str) -> Vec<Token> {
    let lexer = Lexer {
        source_chars: source.chars(),
        source_char_count: source.chars().count(),
        lexeme_start: 0,
        char_reading: 0,
        char_reading_value: '\0',
        line: 0,
        column: 0
    };

    lexer.tokenize()
}

#[allow(unused)]
pub fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        let token_color = match token.token_type {
            TokenType::Number => "\x1b[38;5;230m",
            TokenType::RangeCC => "\x1b[38;5;42m",
            TokenType::RangeOO => "\x1b[38;5;42m",
            TokenType::RangeCO => "\x1b[38;5;42m",
            TokenType::RangeOC => "\x1b[38;5;42m",
            TokenType::Comma => "\x1b[38;5;225m",
            TokenType::Not => "\x1b[38;5;192m",
            TokenType::LBrack => "\x1b[38;5;33m",
            TokenType::RBrack => "\x1b[38;5;33m",
            TokenType::LParen => "\x1b[38;5;141m",
            TokenType::RParen => "\x1b[38;5;141m",
            TokenType::Constraint => "\x1b[38;5;209m",
            TokenType::CMultOf => "\x1b[38;5;195m",
            TokenType::Ignored => "\x1b[0m",
        };

        print!("{}{}\x1b[0m", token_color, token.content);
    }
    std::io::stdout().flush().unwrap();
}