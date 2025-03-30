use constraint::Constraint;
use expression::Expression;

use crate::rng_functions::{random_f64, random_usize};

pub mod expression;
pub mod constraint;

/// A Grand Expression. It is a recursive structure that evaluates a range and modifiers (constraints)
/// or a selection from a list.
/// The range's parameters may be other expressions that have to be evaluated first.
#[derive(Debug, Clone)]
pub struct Gex {
    expression_type: Expression,
    min_number: f64,
    max_number: f64,
    dynamic_constraints: Vec<Constraint>
}

impl Gex {
    pub fn num(num: f64) -> Self {
        Gex {
            expression_type: Expression::Number(num),
            min_number: num,
            max_number: num,
            dynamic_constraints: Vec::new()
        }
    }
    pub fn range(x: Gex, y: Gex, x_open: bool, y_open: bool) -> Self {
        let min_number = x.min_number;
        let max_number = x.max_number;

        // Create box so that we can store it in the expressions
        let x = Box::new(x);
        let y = Box::new(y);
        
        Gex {
            expression_type: Expression::Range(x, y, x_open, y_open),
            min_number,
            max_number,
            dynamic_constraints: Vec::new()
        }
    }
    pub fn select(objects: Vec<Gex>) -> Self {
        // Get minimum and maximum values
        let (min_number, max_number) = objects
            .iter()
            .map(|gex| {
                (gex.min_number, gex.max_number)
            })
            .reduce(|acc, elem| (f64::min(acc.0, elem.0), f64::max(acc.1, elem.1)))
            .expect("range was empty. This should be an Error, not a Panic");

        let objects: Vec<Box<Gex>> = objects.iter().map(|obj| {
            Box::new(obj.to_owned())
        }).collect();
        
        Gex {
            expression_type: Expression::Select(objects),
            min_number,
            max_number,
            dynamic_constraints: Vec::new()
        }
    }

    pub fn eval(&self) -> f64 {
        match &self.expression_type {
            Expression::Number(out) => out.clone(),
            Expression::Range(gex_x, gex_y, x_open, y_open) => Self::eval_range(gex_x.eval(), gex_y.eval(), *x_open, *y_open),
            Expression::Select(items) => Self::eval_select(items.iter().map(|gex| {
                gex.eval()
            }).collect()),
            Expression::PrecalculatedRange(gex_x, gex_y, x_open, y_open, possible_vals) => {
                Self::eval_precalculated(gex_x.eval(), gex_y.eval(), *x_open, *y_open, possible_vals)
            }
        }
    }

    pub fn eval_range(x: f64, y: f64, _x_open: bool, _y_open: bool) -> f64 {
        let number = random_f64(x, y);
        // println!("DEBUG: eval() returned {number}");
        number
    }

    pub fn eval_select(options: Vec<f64>) -> f64 {
        let random_index = random_f64(0f64, options.len() as f64) as usize;
        return options.get(random_index).expect("Out of range. Random index generated was incorrect").clone()
    }

    /*
     * Used in constraints whenever the range is small enough to be inside
     * the memory budget (configurable, probably 1MB-ish by default).
     * 
     * The possible_vals array is assumed to be sorted.
     */
    pub fn eval_precalculated(x: f64, y: f64, x_open: bool, y_open: bool, possible_vals: &Vec<f64>) -> f64 {
        let min_index = possible_vals.iter().position(|num| {
            if x_open {
                *num > x
            } else {
                *num >= x
            }
        }).expect("Minimum possible value withing range not found. Should be impossible?");

        let mut max_index = possible_vals.iter().position(|num| {
            // We return the position of the first INvalid value.
            // The last valid value is the one in the previous index.
            *num >= y
        }).expect("Maximum possible value withing range not found. Should be impossible?");
        max_index -= 1;
        // If our current max index is the same as the maximum value expected and this is
        // an open interval we can't take this number, we should take the previous one.
        if *possible_vals.get(max_index).unwrap() == y && y_open {
            max_index -= 1;
        }

        // Now we get an index within the range and return the precalculated value at that position
        let index = random_usize(min_index, max_index);
        *possible_vals.get(index).unwrap()
    }
}