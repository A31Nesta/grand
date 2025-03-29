use expression::Expression;

use crate::random_f64;

mod expression;

/// A Grand Expression. It is a recursive structure that evaluates a range and modifiers (constraints)
/// or a selection from a list.
/// The range's parameters may be other expressions that have to be evaluated first.
pub struct Gex<'a> {
    expr: Expression<'a>
}

impl<'a> Gex<'a> {
    pub fn eval(&self) -> f64 {
        match &self.expr {
            Expression::Number(out) => out.clone(),
            Expression::RangeCC(gex_x, gex_y) => Self::range(gex_x.eval(), gex_y.eval(), false, false),
            Expression::RangeCO(gex_x, gex_y) => Self::range(gex_x.eval(), gex_y.eval(), false, true),
            Expression::RangeOC(gex_x, gex_y) => Self::range(gex_x.eval(), gex_y.eval(), true, false),
            Expression::RangeOO(gex_x, gex_y) => Self::range(gex_x.eval(), gex_y.eval(), true, true),
            Expression::Select(items) => Self::select(items.iter().map(|gex| {
                gex.eval()
            }).collect()),
        }
    }

    pub fn range(x: f64, y: f64, x_open: bool, y_open: bool) -> f64 {
        let number = random_f64(x, y);
        number
    }

    pub fn select(options: Vec<f64>) -> f64 {
        let random_index = random_f64(0f64, options.len() as f64) as usize;
        return options.get(random_index).expect("Out of range. Random index generated was incorrect").clone()
    }
}