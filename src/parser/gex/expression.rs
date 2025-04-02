use rust_decimal::Decimal;

use super::Gex;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(Decimal),
    Range(Box<Gex>, Box<Gex>, bool, bool), // X, Y, X is Open, Y is Open
    Select(Vec<Box<Gex>>),
    PrecalculatedRange(Box<Gex>, Box<Gex>, bool, bool, Vec<Decimal>), // X, Y, X is Open, Y is Open, possible values
}