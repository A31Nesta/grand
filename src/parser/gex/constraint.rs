use rust_decimal::Decimal;

/*
 * Used for constrained ranges that are too large to be pre-calculated
 * or for ranges with constraints that depend on non-static values, therefore
 * making precalculation impossible.
 */
#[derive(Debug, Clone)]
pub enum Constraint {
    MultipleOf(Decimal), // Expects the Lowest Common Multiple of all the numbers. No expressions allowed here
    NotMultipleOf(Vec<Decimal>), // Expects all the blacklisted numbers
}