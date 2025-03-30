use gex::{constraint::Constraint, Gex};
use parse_error::ParseError;
use token::Token;
use token_type::TokenType;

mod token_type;
mod token;

mod lexer;

pub mod parse_error;
pub mod gex;

pub fn parse(source: &str) -> Result<Gex, ParseError> {
    let tokens = lexer::tokenize(source);
    lexer::print_tokens(&tokens);
    parse_expression(&tokens, 0).0
}

/*
 * The entire code is a single expression.
 * This expression can contain sub-expressions.
 * This function returns a Gex with all the sub-expressions
 * already included by using recursion.
 */
fn parse_expression(tokens: &[Token], mut index: usize) -> (Result<Gex, ParseError>, usize) {
    // At the beginning of an expression we expect:
    // - A number (Gex with Expression of type Number)
    // - A Range operator followed by a number or sub-expression
    // - A sub-expression. This is indicated by a parenthesis.
    //    We need to increment a counter when we see a LParen, decrement it with every RParen
    //    and call this function (parse_expression()) with the slice between parenthesis as parameter
    // - A Selection. This is indicated by a bracket. It's parsed in a similar way as sub-expressions,
    //    but this time we call parse_selection().

    match tokens[index].token_type {
        token_type::TokenType::Number => {
            let x_num: f64 = tokens[index].content.parse().expect("lexing/parsing error. NaN found in numerical token");
            let x = Gex::num(x_num);
            
            index += 1;
            // If there are more tokens we keep parsing this as a range.
            // If there are none, we just return the number
            if tokens.len() > index {
                let res = parse_range(x, tokens, index);
                // TODO: Check if all tokens were actually consumed
                return res;
            } else {
                return (Ok(x), index);
            }
        },
        token_type::TokenType::RangeCC |
        token_type::TokenType::RangeOO |
        token_type::TokenType::RangeCO |
        token_type::TokenType::RangeOC |
        token_type::TokenType::LBrack => {
            let x = Gex::num(i64::MIN as f64);
            let res = parse_range(x, tokens, index);
            // TODO: Check if all tokens were actually consumed
            return res;
        },
        token_type::TokenType::LParen => {
            let subex_end_index = find_subexpression_end(tokens, index);
            let (res, _) = parse_expression(&tokens[index+1..subex_end_index], 0);

            // If there was an error we propagate it
            if let Err(_) = &res {
                return (res, index);
            }
            // Otherwise we continue reading (or return the Gex if we're done)
            index = subex_end_index+1;
            if tokens.len() > index {
                let parsed_range = parse_range(res.unwrap(), tokens, index);
                // TODO: Check if all tokens were actually consumed
                return parsed_range;
            } else {
                return (Ok(res.unwrap()), index);
            }
        },
        _ => {
            // Throw Unexpected Token error
            return (Err(ParseError::UnexpectedToken(
                vec![TokenType::Number, TokenType::RangeCC, TokenType::RangeCO, TokenType::RangeOC, TokenType::RangeOO, TokenType::LBrack, TokenType::LParen],
                tokens[index].token_type.clone(),
                tokens[index].line,
                tokens[index].column
            )), index)
        }
    }
}

fn parse_selection(tokens: &[Token], mut index: usize) -> (Result<Gex, ParseError>, usize) {
    todo!()
}

/*
 * Takes the first Gex (x) and the tokens and index for the next 
 */
fn parse_range(x: Gex, tokens: &[Token], mut index: usize) -> (Result<Gex, ParseError>, usize) {
    let (x_open, y_open) = match tokens[index].token_type {
        TokenType::RangeCC => (false, false),
        TokenType::RangeOO => (true, true),
        TokenType::RangeCO => (false, true),
        TokenType::RangeOC => (true, false),
        _ => {
            return (Err(ParseError::UnexpectedToken(
                vec![TokenType::RangeCC, TokenType::RangeCO, TokenType::RangeOC, TokenType::RangeOO],
                tokens[index].token_type.clone(),
                tokens[index].line,
                tokens[index].column
            )), index)
        }
    };

    // Next should be a number, sub-expression or selection
    index += 1;
    let y: Gex = if tokens.len() > index {
        // If we still have tokens we continue to read. We expect a number (or subexpression / selection)
        let token_y = &tokens[index];
        match &token_y.token_type {
            TokenType::Number => {
                let y_num: f64 = token_y.content.parse().expect("lexing/parsing error. NaN found in numerical token");
                index += 1;
                Gex::num(y_num)
            }
            TokenType::LBrack => {
                // TODO: implement parse_selection()
                todo!()
            }
            TokenType::LParen => {
                let subex_end_index = find_subexpression_end(tokens, index);
                let (res, _) = parse_expression(&tokens[index+1..subex_end_index], 0);
                index = subex_end_index+1;
                match res {
                    Ok(gex) => gex,
                    Err(err) => return (Err(err), index),
                }
            }
            _ => Gex::num(i64::MAX as f64)
        }
    } else {
        // Ok so we have no more tokens to read, let's use the default value
        Gex::num(i64::MAX as f64)
    };

    if tokens.len() > index {
        parse_constraints(Gex::range(x, y, x_open, y_open), tokens, index)
    } else {
        (Ok(Gex::range(x, y, x_open, y_open)), index)
    }
}

fn parse_constraints(mut gex: Gex, tokens: &[Token], mut index: usize) -> (Result<Gex, ParseError>, usize) {
    let constraints: Vec<Constraint> = Vec::new();

    while index < tokens.len() {
        let token = &tokens[index];

        // Exit if this is not a constraint
        if token.token_type != TokenType::Constraint {
            return (
                Err(ParseError::UnexpectedToken(
                    vec![TokenType::Constraint],
                    token.token_type.clone(),
                    token.line,
                    token.column
                )),
                index
            )
        }
        index += 1;

        // Parse constraint
        // We expect a NOT or a Constraint
        // If we get a NOT, we expect a Constraint afterwards
        let mut constraint = match tokens[index].token_type {
            TokenType::Not => {
                index += 1;
                // TODO: Change with match if this ever gets expanded. Alternatively, refactor to avoid duplication
                if tokens[index].token_type != TokenType::CMultOf {
                    return (
                        Err(ParseError::UnexpectedToken(
                            vec![TokenType::CMultOf],
                            token.token_type.clone(),
                            token.line,
                            token.column
                        )),
                        index
                    )
                }
                index += 1;
                Constraint::NotMultipleOf(Vec::new())
            },
            TokenType::CMultOf => {
                index += 1;
                Constraint::MultipleOf(0f64)
            },
            _ => {
                return (
                    Err(ParseError::UnexpectedToken(
                        vec![TokenType::Not, TokenType::CMultOf],
                        token.token_type.clone(),
                        token.line,
                        token.column
                    )),
                    index
                )
            }
        };

        // Find numbers
        let mut entries: Vec<f64> = Vec::new();
        let mut expecting_number = true; // we expect comma, number, comma, number..... After number we could also have another constraint

        'loop1: loop {
            if expecting_number {
                if tokens[index].token_type != TokenType::Number {
                    return (
                        Err(ParseError::UnexpectedToken(
                            vec![TokenType::Number],
                            token.token_type.clone(),
                            token.line,
                            token.column
                        )),
                        index
                    )
                }

                let number: f64 = tokens[index].content.parse().expect("lexing/parsing error. NaN found in numerical token");
                entries.push(number);
            } else {
                if index >= tokens.len() {
                    break;
                }
                match tokens[index].token_type {
                    TokenType::Comma => (), // Continue the loop as usual
                    TokenType::Constraint => {
                        println!("AA");
                        break 'loop1;
                    },
                    _ => return (
                        Err(ParseError::UnexpectedToken(
                            vec![TokenType::Comma, TokenType::Constraint],
                            tokens[index].token_type.clone(),
                            tokens[index].line,
                            tokens[index].column
                        )),
                        index
                    )
                }
            }
            expecting_number = !expecting_number;
            index += 1;
        }

        match &mut constraint {
            Constraint::MultipleOf(lcm) => {
                *lcm = least_common_multiple(&mut entries);
            },
            Constraint::NotMultipleOf(items) => {
                *items = entries;
            },
        }
        println!("{:?}", constraint)
    }

    (Ok(gex), index)
}

fn find_subexpression_end(tokens: &[Token], mut index: usize) -> usize {
    let mut counter: usize = 0;
    // This first token should be a LParen, increment count by 1 if this is the case.
    // When this first token is not a parenthesis we would exit immediately
    if let TokenType::LParen = tokens[index].token_type {
        counter += 1;
    }

    while counter > 0 {
        index += 1;
        // Increment counter when we encounter a new sub-expression,
        // decrement it when we encounter the end of a sub-expression.
        // If we reach 0 we found the end of our sub-expression.
        match tokens[index].token_type {
            TokenType::LParen => counter += 1,
            TokenType::RParen => counter -= 1,
            _ => {}
        }
    }

    index
}

fn least_common_multiple(numbers: &Vec<f64>) -> f64 {
    todo!()
}