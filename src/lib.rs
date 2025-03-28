mod rng_traits;
mod rng_functions;

mod parser;

use parser::parse;
pub use rng_traits::Randomizable;
pub use rng_functions::random_f64;


pub fn expr(expression: &str) {
    parse(expression);
}