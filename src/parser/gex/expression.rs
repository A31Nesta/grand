use super::Gex;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    Range(Box<Gex>, Box<Gex>, bool, bool), // X, Y, X is Open, Y is Open
    Select(Vec<Box<Gex>>),
    PrecalculatedRange(Box<Gex>, Box<Gex>, bool, bool, Vec<f64>), // X, Y, X is Open, Y is Open, possible values
}