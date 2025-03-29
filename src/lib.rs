//! # Grand Expressions
//! 
//! **Grand** (Glorified `rand()`) **Expressions** are an easy way
//! to obtain random numbers with certain constraints or characteristics.
//! 
//! ## How do they look like?
//! 
//! Grand Expressions are based on Regular Expressions so they look relatively
//! similar.
//! Grand Expressions' random number generation is done with Ranges:
//! 
//! - `0..10` - Generates a number between 0 and 10 (inclusive)
//! - `0,,10` - Generates a number between 0 and 10 (exclusive, will not be exactly 0 or 10)
//! - `-10,.10` - Generates a number between -10 and 10. This number can't be exactly -10 but can be 10
//! - `0..` - Generates a random positive number that could also be 0
//! - `..0` - Generates a random negative number that could also be 0
//! - `..` - Generates a random number
//! 
//! ### Constraints
//! 
//! What makes Grand Expressions interesting is not just the comfortable RNG
//! system, it's the constraints. You can force the generated number to have specific
//! characteristics:
//! 
//! - `0..100|*2` - The pipe (`|`) indicates a constraint and the asterisk (`*`) indicates a "multiple of" constraint.
//!     In this case we are generating a random number between 0 and 100 that is a multiple of 2 (even).
//! - `0..100|!*2` - We can also negate the constraint. This expression generates a random odd number between 0 and 100.
//! - `0..100|*2,3,5` - This constraint indicates that the number must be a multiple of 2, 3 and 5.


mod rng_traits;
mod rng_functions;

mod parser;

use parser::parse;
pub use rng_traits::Randomizable;
pub use rng_functions::random_f64;


pub fn expr(expression: &str) {
    parse(expression);
}