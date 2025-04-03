# Grand

A Glorified `rand()` function. It's like RegEx but for numbers... kinda?

More specifically a constraint-based random number generator in the form
of a JIT-compiled language.

## Basic Usage:

Grand evaluates expressions (Grand Expressions) in order to generate the numbers. These expressions are based on ranges that can generate numbers that also follow specific constraints.

```rust
fn main() {
    // Generate a number from 1 to 10 (inclusive)
    let one_to_ten = grand::compile("1..10").generate();

    // Any multiple of 2
    let even_num = grand::compile("..|*2").generate();

    /*
        Generate a number between (but not equal to) a random
        number betwen 0 and 10 and a random number between 20 
        and (exclusively) 50 that is a multiple of 2.

        This number must be a multiple of 2, 3 and 5. The Least
        Common Multiple is calculated at compile-time (compile() function)
    */
    let generator: grand::GrandEx = grand::compile("((0..10),,(20.,50|*2))|*2,3,5");

    // Generates 10 numbers using the compiled expression.
    for _ in 0..10 {
        let n = generator.generate();
        println!("Result: {n}");
    }
}
```

- `grand::compile()` takes a string slice and returns a `GrandEx` object containing the "compiled" expression. Some minimal optimizations are done if possible.

- `grand::GrandEx::generate()` runs the expression and returns the random number generated.

## Building

Run this command to build the library for WASM:
```bash
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --target web
```