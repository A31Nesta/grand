use super::Gex;

pub enum Expression<'a> {
    Number(f64),

    RangeCC(&'a Gex<'a>, &'a Gex<'a>),
    RangeCO(&'a Gex<'a>, &'a Gex<'a>),
    RangeOC(&'a Gex<'a>, &'a Gex<'a>),
    RangeOO(&'a Gex<'a>, &'a Gex<'a>),

    Select(Vec<&'a Gex<'a>>)
}