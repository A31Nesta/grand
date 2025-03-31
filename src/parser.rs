use gex::{constraint::Constraint, Gex};
use parse_error::ParseError;
use token::Token;
use token_type::TokenType;

mod token_type;
mod token;

mod lexer;

pub mod parse_error;
pub mod gex;

const PRECALC_MEMORY_BUDGET: f64 = 131072f64; // 1 MB Maximum

/*
 * TODO: Refactor entirely
 * 
 * CURRENT PROBLEMS:
 * - Heavy Code duplication
 * - Lacking features (expressions in Selections)
 * 
 * SOLUTION:
 * - More general parse_eval() that calls other functions like parse_range() or parse_selection().
 * - Heavier use of this improved parse_eval(). Useful inside parse_range() and parse_selection().
 * - Utility functions for reading arrays of numbers.
 *     - Option to allow or disallow sub-expressions (number-only mode for constraints)
 * - Add configuration
 * - Add errors and error handling. Getting a random number shouldn't (usually) panic.
 */

pub fn parse(source: &str) -> Gex {
    let tokens = lexer::tokenize(source);
    lexer::print_tokens(&tokens);
    parse_expression(&tokens, 0).0.unwrap()
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
            let x = Gex::from_num(x_num);
            
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
            let subex_end_index = find_selection_end(tokens, index);
            let res = parse_selection(&tokens[index+1..subex_end_index]);
            index = subex_end_index+1;
            // TODO: Check if all tokens were actually consumed
            return (res, index);
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

// Refactor into READ_VECTOR
fn parse_selection(tokens: &[Token]) -> Result<Gex, ParseError> {
    let mut index = 0;
    // Find numbers
    let mut entries: Vec<Gex> = Vec::new();
    let mut expecting_number = true; // we expect comma, number, comma, number..... After number we could also have another constraint

    'loop1: loop {
        if expecting_number {
            // TODO: Allow the use of other expressions
            if tokens[index].token_type != TokenType::Number {
                return Err(ParseError::UnexpectedToken(
                    vec![TokenType::Number],
                    tokens[index].token_type.clone(),
                    tokens[index].line,
                    tokens[index].column
                ))
            }

            let number: f64 = tokens[index].content.parse().expect("lexing/parsing error. NaN found in numerical token");
            entries.push(Gex::from_num(number));
        } else {
            if index >= tokens.len() {
                break;
            }
            match tokens[index].token_type {
                TokenType::Comma => (), // Continue the loop as usual
                TokenType::Constraint => {
                    break 'loop1;
                },
                _ => return Err(ParseError::UnexpectedToken(
                    vec![TokenType::Comma, TokenType::Constraint],
                    tokens[index].token_type.clone(),
                    tokens[index].line,
                    tokens[index].column
                ))
            }
        }
        expecting_number = !expecting_number;
        index += 1;
    }

    Ok(Gex::from_select(entries))
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
                Gex::from_num(y_num)
            }
            TokenType::LBrack => {
                let subex_end_index = find_selection_end(tokens, index);
                let res = parse_selection(&tokens[index+1..subex_end_index]);
                index = subex_end_index+1;
                match res {
                    Ok(gex) => gex,
                    Err(err) => return (Err(err), index),
                }
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
            _ => Gex::from_num(i64::MAX as f64)
        }
    } else {
        // Ok so we have no more tokens to read, let's use the default value
        Gex::from_num(i64::MAX as f64)
    };

    if tokens.len() > index {
        parse_constraints(Gex::from_range(x, y, x_open, y_open), tokens, index)
    } else {
        (Ok(Gex::from_range(x, y, x_open, y_open)), index)
    }
}

/*
 * TODO: NEEDS IMMEDIATE REFACTORING
 */
fn parse_constraints(mut gex: Gex, tokens: &[Token], mut index: usize) -> (Result<Gex, ParseError>, usize) {
    let mut c_mult_of: Option<f64> = None;
    let mut c_not_mult_of: Option<Vec<f64>> = None;

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
                *lcm = least_common_multiple(&entries);
                c_mult_of = Some(*lcm);
            },
            Constraint::NotMultipleOf(items) => {
                *items = entries;
                c_not_mult_of = Some(items.clone());
            },
        }
    }

    // Precalc
    if c_mult_of.is_some() && c_not_mult_of.is_some() {
        let res = precalculate_constraint(gex.clone(), c_mult_of.unwrap(), c_not_mult_of.clone().unwrap(), gex.min_number(), gex.max_number());
        if res.is_some() {
            gex = res.unwrap()
        } else {
            gex.add_constraint(Constraint::MultipleOf(c_mult_of.unwrap()));
            gex.add_constraint(Constraint::NotMultipleOf(c_not_mult_of.unwrap()));
        }
    } else {
        if let Some(mult_of) = c_mult_of {
            gex.add_constraint(Constraint::MultipleOf(mult_of));
        } else if let Some(not_mult_of) = c_not_mult_of {
            gex.add_constraint(Constraint::NotMultipleOf(not_mult_of));
        }
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
fn find_selection_end(tokens: &[Token], mut index: usize) -> usize {
    let mut counter: usize = 0;
    if let TokenType::LBrack = tokens[index].token_type {
        counter += 1;
    }
    while counter > 0 {
        index += 1;
        match tokens[index].token_type {
            TokenType::LBrack => counter += 1,
            TokenType::RBrack => counter -= 1,
            _ => {}
        }
    }
    index
}

fn least_common_multiple(numbers: &[f64]) -> f64 {
    // Return this number if the length is 1
    if numbers.len() == 0 { panic!("LCM of 0 numbers?? WTF?") }
    if numbers.len() == 1 { return numbers[0] }

    let half_len = numbers.len() / 2;
    let x = least_common_multiple(&numbers[0..half_len]); // Calculate GCD with half of the array
    let y = least_common_multiple(&numbers[half_len..numbers.len()]); // Calculate with other half

    let gcd = greatest_common_denominator(x, y);

    (x * y) / gcd
}

fn greatest_common_denominator(x: f64, y: f64) -> f64 {
    // Order
    let max_xy = x.max(y);
    let min_xy = x.min(y);

    // Euclidean Algorithm
    let mut bigger = max_xy;
    let mut smaller = min_xy;
    while smaller != 0f64 {
        let rem = bigger % smaller;
        bigger = smaller;
        smaller = rem;
    }

    // The smaller is 0 now, the previous value is the GCD
    bigger
}

/**
 * Returns None if the range is too big for the budget or if there are no possible values
 */
fn precalculate_constraint(orig: Gex, multiple_of: f64, not_multiple_of: Vec<f64>, start: f64, end: f64) -> Option<Gex> {
    let max_byte_count = calc_max_constraint_size(multiple_of, end - start);
    println!("MAX MEMORY: {}/{} bytes", max_byte_count, PRECALC_MEMORY_BUDGET);
    if max_byte_count > PRECALC_MEMORY_BUDGET {
        println!("Warning: Max byte count for this range is {}. Budget is {} bytes", max_byte_count, PRECALC_MEMORY_BUDGET);
        return None
    }

    let mut possible_value: Vec<f64> = Vec::new();
    let mut current = (start / multiple_of).floor() * multiple_of;

    while current <= end  {
        // Check for blacklisted multiples
        let mut is_mult = false;
        for number in &not_multiple_of {
            if current % number == 0f64 { is_mult = true; }
        }
        // If it's within the constraint we're done
        if !is_mult {
            possible_value.push(current);
        }
        current += multiple_of;
    }

    if possible_value.is_empty() {
        return None;
    }

    let gex = Gex::from_precalc(orig, possible_value);
    Some(gex)
}

fn calc_max_constraint_size(multiple_of: f64, range: f64) -> f64 {
    (range / multiple_of) * size_of::<f64>() as f64
}