use std::error::Error;

use grand::random_f64;

fn main() -> Result<(), Box<dyn Error>> {
    let num = random_f64();
    println!("{num}");

    Ok(())
}