use token::Token;

mod token_type;
mod token;

mod lexer;

mod gex;

pub fn parse(source: &str) {
    let tokens = lexer::tokenize(source);
    lexer::print_tokens(&tokens);

    
}

/*
 * The entire code is a single expression.
 * This expression can contain sub-expressions.
 * This function returns a Gex with all the sub-expressions
 * already included by using recursion.
 */
pub fn parse_expression(tokens: &[Token]) {
    // At the beginning of an expression we expect:
    // - A number (Gex with Expression of type Number)
    // - A Range operator followed by a number or sub-expression
    // - A sub-expression. This is a parenthesis.
    //    We need to increment a counter when we see a LParen, decrement it with every RParen
    //    and call this function (parse_expression()) with the slice between parenthesis as parameter
    // - A Selection. This is a bracket
}