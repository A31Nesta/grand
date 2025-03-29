use expression::Expression;

use crate::random_f64;

pub mod expression;

/// A Grand Expression. It is a recursive structure that evaluates a range and modifiers (constraints)
/// or a selection from a list.
/// The range's parameters may be other expressions that have to be evaluated first.
#[derive(Debug, Clone)]
pub struct Gex {
    expression_type: Expression,
}

impl Gex {
    pub fn num(num: f64) -> Self {
        Gex {
            expression_type: Expression::Number(num)
        }
    }
    pub fn range(x: Gex, y: Gex, x_open: bool, y_open: bool) -> Self {
        // Create box so that we can store it in the expressions
        let x = Box::new(x);
        let y = Box::new(y);
        // Create expression type
        let expression_type = if x_open {
            if y_open {
                Expression::RangeOO(x, y)
            } else {
                Expression::RangeOC(x, y)
            }
        } else {
            if y_open {
                Expression::RangeCO(x, y)
            } else {
                Expression::RangeCC(x, y)
            }
        };
        Gex { expression_type }
    }

    pub fn eval(&self) -> f64 {
        match &self.expression_type {
            Expression::Number(out) => out.clone(),
            Expression::RangeCC(gex_x, gex_y) => Self::eval_range(gex_x.eval(), gex_y.eval(), false, false),
            Expression::RangeCO(gex_x, gex_y) => Self::eval_range(gex_x.eval(), gex_y.eval(), false, true),
            Expression::RangeOC(gex_x, gex_y) => Self::eval_range(gex_x.eval(), gex_y.eval(), true, false),
            Expression::RangeOO(gex_x, gex_y) => Self::eval_range(gex_x.eval(), gex_y.eval(), true, true),
            Expression::Select(items) => Self::eval_select(items.iter().map(|gex| {
                gex.eval()
            }).collect()),
        }
    }

    pub fn eval_range(x: f64, y: f64, _x_open: bool, _y_open: bool) -> f64 {
        let number = random_f64(x, y);
        println!("DEBUG: eval() returned {number}");
        number
    }

    pub fn eval_select(options: Vec<f64>) -> f64 {
        let random_index = random_f64(0f64, options.len() as f64) as usize;
        return options.get(random_index).expect("Out of range. Random index generated was incorrect").clone()
    }
}