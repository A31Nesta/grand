# Grand

A Glorified `rand()` function.

More specifically a constraint-based random number generator.

## Basic Usage:

Grand evaluates expressions (Grand Expressions) in order to generate the numbers. These expressions are based on ranges that can generate numbers that also follow specific constraints.

Since this is a work in progress, this is a mockup:

```rust
fn main() {
    // Generate a number from 1 to 10 (inclusive)
    let one_to_ten: i32 = grand::expr("1..10").eval();

    // Any multiple of 2
    let even_num: i64 = grand::expr("..|*2|").eval();

    /*
        Generate a number between (but not equal to) a random
        number betwen 0 and 10 and a random number between 20 
        and (exclusively) 50 that is a multiple of 2.

        This number must be a multiple of a number randomly
        selected between 2, 3 or 5.
    */
    let other_num: f64 = grand::expr("((0..10),,(20.,50|*2|))|*[2,3,5]|").eval();
}
```

- `grand::expr()` takes a string slice and returns a `Gex` (Grand Expression) object containing the "compiled" expression. Some minimal optimizations are done if possible.

- `grand::Gex::eval()` runs the expression and returns the random number generated.