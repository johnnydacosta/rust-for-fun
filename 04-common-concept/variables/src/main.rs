// Constants must be annotated with a type.
// They are valid for the entire duration of the program and can be declared in any scope.
const OUTER_PI: f32 = 3.14;

fn main() {
    // --- Mutability ---

    // Variables are immutable by default in Rust.
    // The `mut` keyword explicitly opts into mutability, signaling to the compiler
    // and other developers that this value will change.
    //
    // let x = 4; // Will generate an error
    let mut x = 4;
    println!("The value of x is {x}");

    x += 1;
    println!("The new value of x is {x}");

    // --- Constants ---

    // Constants are not just immutable by default; they are ALWAYS immutable.
    // They cannot be used with `mut` and must be set to a constant expression,
    // not a value that could only be computed at runtime.
    const PI: f32 = 3.14;
    println!("The value of PI is {PI}");
    println!("The value of OUTER_PI is {OUTER_PI}");

    // --- Shadowing ---

    // Shadowing allows us to redeclare a variable with the `let` keyword.
    // This effectively "masks" the previous variable in the current scope.
    let y = 5;
    let y = y + 1;
    println!("The value of y is {y}");

    // Shadowing is block-scoped.
    // The 'inner' y only exists until the end of this curly brace.
    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }

    // After the block, the previous shadowing of y remains in effect.
    println!("The value of y after the inner scope {y}");

    // A major advantage of shadowing over `mut` is the ability to change types
    // while keeping a descriptive variable name.
    let spaces = "    "; // type: &str
    let spaces = spaces.len(); // type: usize
    println!("The desired space length is {spaces}");
}
