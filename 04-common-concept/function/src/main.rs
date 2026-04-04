fn main() {
    run();
}

fn run() {
    println!("Hello, world!");
    let x = 5;

    // In Rust, we pass arguments to functions just like most languages.
    print_value(x);
    print_labeled_measurement(x, 'h');

    // --- Expressions vs. Statements ---

    // A block surrounded by curly braces is an EXPRESSION.
    // Expressions evaluate to a value and do NOT end with a semicolon.
    let y = {
        let x = 2; // This is a statement (ends in semicolon)
        x + 1 // This is an expression. No semicolon = "return this value"
    };
    println!("The value of y is: {y}");

    let sum_xy = add(x, y);
    println!("The addition of {x} and {y} is: {sum_xy}");
}

// Function signatures MUST declare the type of each parameter.
fn print_value(x: i32) {
    println!("The value of x is: {x}");
}

// For multiple parameters, separate them with commas.
fn print_labeled_measurement(value: i32, unit: char) {
    println!("The measurement is: {value}{unit}");
}

// --- Return Values ---

// The '->' arrow indicates the return type.
fn add(x: i32, y: i32) -> i32 {
    // Rust is an expression-based language.
    // The last line of a function is implicitly returned if you omit the semicolon.
    x + y

    // You could also write: return x + y;
    // But omitting 'return' is the idiomatic "Rust way" for the final value.
}
