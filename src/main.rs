use std::error::Error;

use grand::expr;

fn main() -> Result<(), Box<dyn Error>> {
    // let gex = expr("(0..10),,(20.,50|*2|)|*[2,3,5]");
    let gex = expr("0..(20..50)")?;
    let res = gex.eval();

    println!("Result: {res}");

    Ok(())
}