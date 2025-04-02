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
//! 
//! ### Performance
//! 
//! Using constant (hard-coded) numbers in constraints and ranges with small amounts of possible values makes the compiler
//! store all possible values of the range (including constraints) in memory, making the generation very fast.  
//! This can take a lot of memory (1MB-ish max maybe?).
//! 
//! Constraints with sub-expressions make pre-calculation impossible, since we can't know what the constraint will at runtime
//! while we are compiling. This (and extremely large ranges that would be beyond the memory budget) makes constraints work
//! in a different way:
//! 
//! #### Gambling and hoping to get a good value
//! 
//! Imagine that we want a number that is a multiple of X and not a multiple of Y.  
//! We can try to get a random number that is a multiple of X and check if it's a multiple of Y. If it is, we generate another,
//! if it isn't, we just return the value.  
//! What if we end up in an endless loop? After several attempts, the program stops trying to get a number and throws an error.
//! Unfortunately, this makes these dynamic constraints unreliable.


mod rng_traits;
mod rng_functions;

mod parser;

use parser::parse;

pub use rng_traits::Randomizable;
pub use rng_functions::random_decimal;
pub use parser::gex::Gex;
pub use parser::parse_error::ParseError;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct GrandEx {
    gex: Gex
}

#[wasm_bindgen]
impl GrandEx {
    pub fn run(&self) -> f64 {
        self.gex.run()
    }
}

#[wasm_bindgen]
pub fn compile(expression: &str) -> GrandEx {
    GrandEx { gex: parse(expression) }
}