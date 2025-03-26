use std::error::Error;

use grand::rng::Randomizable;

fn main() -> Result<(), Box<dyn Error>> {
    let mut has_min = false;
    let mut has_max = false;

    const MIN: u16 = 32;
    const MAX: u16 = 42069;

    loop {
        let res = u16::random_range(MIN, MAX)?;
        print!("{res}, ");
        if *res == MAX { has_max = true }
        if *res == MIN { has_min = true }
        if has_max && has_min { break }
        
        if *res < MIN { panic!() }
        if *res > MAX { panic!() }
    }

    let res = u16::random_range(MIN, MAX)?;
    println!("{res}");
    println!("Done!");

    Ok(())
}