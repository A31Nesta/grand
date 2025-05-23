use std::error::Error;

use grand::compile;

fn main() -> Result<(), Box<dyn Error>> {
    // let gex = compile("(0..10),,(20.,50|*2)|*3,5")?;
    let gex = compile("-10.5..0.5|*0.25");
    
    for _ in 0..10 {
        let res = gex.generate();
        println!("Result: {res}");
    }

    Ok(())
}