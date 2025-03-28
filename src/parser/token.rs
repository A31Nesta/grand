use super::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub content: String,
    pub line: usize,
    pub column: usize
}