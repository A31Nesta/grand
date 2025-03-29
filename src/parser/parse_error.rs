use std::{error::Error, fmt::Display};

use super::token_type::TokenType;

/// Error type for parser errors. They can be:
/// 
/// ## UnexpectedToken
/// 
/// It has 4 arguments:
/// - `Vec<TokenType>` with the expected token(s)
/// - `TokenType` with the actual token in the code
/// - `usize` with the line where the error happened (most likely one since expressions are short)
/// - `usize` with the column where the error happened
#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken(Vec<TokenType>, TokenType, usize, usize)
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(valid_tokens, actual_token, line, column) => {
                write!(f, "Unexpected Token in line {}, column {}. Expected one of {:?}, found {:?}", line, column, valid_tokens, actual_token)
            },
        }
    }
}

impl Error for ParseError {}