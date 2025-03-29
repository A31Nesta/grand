use super::Gex;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),

    RangeCC(Box<Gex>, Box<Gex>),
    RangeCO(Box<Gex>, Box<Gex>),
    RangeOC(Box<Gex>, Box<Gex>),
    RangeOO(Box<Gex>, Box<Gex>),

    Select(Vec<Box<Gex>>)
}