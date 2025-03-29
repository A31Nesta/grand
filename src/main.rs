use std::error::Error;

use grand::expr;

fn main() -> Result<(), Box<dyn Error>> {
    expr("(0..10),,(20.,50|*2|)|*[2,3,5]");

    Ok(())
}