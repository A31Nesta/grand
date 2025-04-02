use constraint::Constraint;
use expression::Expression;
use rust_decimal::{dec, Decimal};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{random_decimal, rng_functions::{random_decimal_int, random_usize}};

pub mod expression;
pub mod constraint;

const MAX_RANGE_HELL_REROLLS: usize = 1000;

/// A Grand Expression. It is a recursive structure that evaluates a range and modifiers (constraints)
/// or a selection from a list.
/// The range's parameters may be other expressions that have to be evaluated first.
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Gex {
    expression_type: Expression,
    min_number: Decimal,
    max_number: Decimal,
    dynamic_constraints: Vec<Constraint>,
    float_mode: bool, // a mult_of constraint with an integer value disables float mode. Integer mode enforces closed/open ranges better
}

impl Gex {
    pub fn from_num(num: Decimal) -> Self {
        Gex {
            expression_type: Expression::Number(num),
            min_number: num,
            max_number: num,
            dynamic_constraints: Vec::new(),
            float_mode: true
        }
    }
    pub fn from_range(x: Gex, y: Gex, x_open: bool, y_open: bool) -> Self {
        let min_number = x.min_number;
        let max_number = y.max_number;

        // Create box so that we can store it in the expressions
        let x = Box::new(x);
        let y = Box::new(y);
        
        Gex {
            expression_type: Expression::Range(x, y, x_open, y_open),
            min_number,
            max_number,
            dynamic_constraints: Vec::new(),
            float_mode: true
        }
    }
    pub fn from_select(objects: Vec<Gex>) -> Self {
        // Get minimum and maximum values and float mode
        // Float mode propagates, if this Gex contains float-mode Gexes, it will become
        // a float-mode Gex as well
        let (min_number, max_number, float_mode) = objects
            .iter()
            .map(|gex| {
                (gex.min_number, gex.max_number, gex.float_mode)
            })
            .reduce(|acc, elem| (Decimal::min(acc.0, elem.0), Decimal::max(acc.1, elem.1), acc.2 || elem.2))
            .expect("range was empty. This should be an Error, not a Panic");

        let objects: Vec<Box<Gex>> = objects.iter().map(|obj| {
            Box::new(obj.to_owned())
        }).collect();

        Gex {
            expression_type: Expression::Select(objects),
            min_number,
            max_number,
            dynamic_constraints: Vec::new(),
            float_mode
        }
    }
    pub fn from_precalc(orig: Gex, values: Vec<Decimal>) -> Self {
        // Get minimum and maximum values
        let min = values.get(0).unwrap().clone();
        let max = values.get(values.len()-1).unwrap().clone();
        let (x_gex, y_gex, x_open, y_open) = if let Expression::Range(xg, yg, xo, yo) = orig.expression_type {
            (xg, yg, xo, yo)
        } else {
            (Box::new(Gex::from_num(min)), Box::new(Gex::from_num(max)), false, false)
        };
        // Get Float mode by searching for a decimal number. If we find it, this Gex should be in float mode
        // TODO: Remove float/integer mode distinction, it introduces more bugs
        let float_mode = 
            match values.iter().find(|value| {
                !value.is_integer()
            }) {
                Some(_) => true,
                None => false,
            }
        ;
        Gex {
            expression_type: Expression::PrecalculatedRange(x_gex, y_gex, x_open, y_open, values),
            min_number: min,
            max_number: max,
            dynamic_constraints: Vec::new(),
            float_mode
        }
    }

    pub fn min_number(&self) -> Decimal {
        self.min_number
    }
    pub fn max_number(&self) -> Decimal {
        self.max_number
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        if let Constraint::MultipleOf(value) = constraint {
            if value.is_integer() {
                // If this number should be a multiple of an integer,
                // float mode should be disabled
                self.float_mode = false;
            }
        }
        self.dynamic_constraints.push(constraint);
    }

    pub fn run(&self) -> Decimal {
        match &self.expression_type {
            Expression::Number(out) => out.clone(),
            Expression::Range(gex_x, gex_y, x_open, y_open) => self.eval_range(gex_x.run(), gex_y.run(), *x_open, *y_open),
            Expression::Select(items) => Self::eval_select(items.iter().map(|gex| {
                gex.run()
            }).collect()),
            Expression::PrecalculatedRange(gex_x, gex_y, x_open, y_open, possible_vals) => {
                Self::eval_precalculated(gex_x.run(), gex_y.run(), *x_open, *y_open, possible_vals)
            }
        }
    }

    fn eval_range(&self, x: Decimal, y: Decimal, x_open: bool, y_open: bool) -> Decimal {
        self.eval_range_hell(x, y, x_open, y_open, 0)
    }
    fn eval_range_hell(&self, x: Decimal, y: Decimal, x_open: bool, y_open: bool, iteration_n: usize) -> Decimal {
        let mut number = if self.float_mode {
            let mut generated = random_decimal(x, y);
            // This should basically never happen, but we have to check if we got exactly
            // X or Y in supposedly open intervals.
            // A bit strange how it's implemented but, again, it should never happen lol
            if x_open && generated == x {
                generated += dec!(0.001);
            }
            else if y_open && generated == y {
                generated -= dec!(0.001);
            }
            generated
        } else {
            // Enforce ranges. This can actually be done with integers
            let x_range_mod = if x_open { Decimal::ONE } else { Decimal::ZERO };
            let y_range_mod = if y_open { Decimal::ZERO } else { Decimal::ONE };
            random_decimal_int(x + x_range_mod, y + y_range_mod)
        };
        
        for constraint in &self.dynamic_constraints {
            match constraint {
                Constraint::MultipleOf(mult_of) => {
                    number = (number / mult_of).floor() * mult_of;
                },
                Constraint::NotMultipleOf(items) => {
                    // Stop infinite rerolls when we tried a bunch of times with no... hehe.. dice
                    if iteration_n > MAX_RANGE_HELL_REROLLS {
                        break;
                    }

                    // try again
                    let mut is_mult: bool = false;
                    for n in items {
                        if number % n == Decimal::ZERO { is_mult = true; }
                    }
                    if is_mult {
                        // Ohhh shit... here we go again
                        number = self.eval_range_hell(x, y, x_open, y_open, iteration_n+1);
                    }
                },
            }
        }

        number
    }

    fn eval_select(options: Vec<Decimal>) -> Decimal {
        let random_index = random_usize(0, options.len());
        return options.get(random_index).expect("Out of range. Random index generated was incorrect").clone()
    }

    /*
     * Used in constraints whenever the range is small enough to be inside
     * the memory budget (configurable, probably 1MB-ish by default).
     * 
     * The possible_vals array is assumed to be sorted.
     */
    fn eval_precalculated(x: Decimal, y: Decimal, x_open: bool, y_open: bool, possible_vals: &Vec<Decimal>) -> Decimal {
        let min_index = possible_vals.iter().position(|num| {
            if x_open {
                *num > x
            } else {
                *num >= x
            }
        }).expect("Minimum possible value within range not found. Should be impossible?");

        let mut max_index = possible_vals.iter().position(|num| {
            // We return the position of the first INvalid value.
            // The last valid value is the one in the previous index.
            *num >= y
        }).unwrap_or(possible_vals.len());
        // If our current max index is the same as the maximum value expected and this is
        // an open interval we can't take this number, we should take the previous one.
        if y_open && *possible_vals.get(max_index-1).unwrap() == y {
            max_index -= 1;
        }

        // Now we get an index within the range and return the precalculated value at that position
        let index = random_usize(min_index, max_index); // from min_index (inclusive) to max_index (without reaching it), this won't overflow
        *possible_vals.get(index).unwrap()
    }
}