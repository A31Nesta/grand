use std::error::Error;

use grand::random_f64;

fn main() -> Result<(), Box<dyn Error>> {
    let num = random_f64(-10.0, 10.0);
    println!("{num}");

    Ok(())
}