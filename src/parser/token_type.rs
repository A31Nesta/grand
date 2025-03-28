#[derive(PartialEq, Debug)]
pub enum TokenType {
    // Numbers and ranges
    Number,
    RangeCC,
    RangeOO,
    RangeCO,
    RangeOC,
    // Separator
    Comma,
    // Control
    LBrack,
    RBrack,
    LParen,
    RParen,
    Constraint,
    // Constraints
    CMultOf,

    // Anything that is not valid is ignored.
    // This allows you to write comments as long
    // as you only use letters and only a few symbols
    Ignored
}