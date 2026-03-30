use std::io;

fn main() {
    // --- Integer Overflow Handling ---
    let mut a: u8 = 255;
    println!("Initial: a={}", a);

    // a = a + 1; // Standard addition will panic in debug mode on overflow.

    // Wrapping: 255 + 10 = 9 (starts over at 0).
    // Use this when you want "clock-like" behavior.
    a = a.wrapping_add(10);
    println!("Wrapping add 10 (255 -> 9): a={}", a);

    // Saturating: 255 + 10 = 255 (stays at the limit).
    // Use this for things like health bars or volume levels.
    a = u8::MAX;
    a = a.saturating_add(10);
    println!("Saturating add 10 (255 -> 255): a={}", a);

    // Note: Rust doesn't have "strict_add"; the standard '+' is 'strict'
    // because it panics on overflow in debug builds.

    // --- Floating Point Precision ---
    let a = 1.0;
    let b = 0.2; // Changed to 0.2 to demonstrate the classic float precision issue
    let sum = a + b;

    // WARNING: Comparing floats with '==' is risky due to rounding errors.
    // 1.0 + 0.2 might be 1.2000000000000002.
    let expected_res = 1.2 == sum;
    println!("The sum of {a} and {b} is {sum}");
    println!("Is 1.2 exactly equal to sum? {expected_res}");

    // --- Scalar Types (Integers) ---
    let a = 1; // Default integer type is i32 (32-bit signed)
    let b: i32 = -2;
    println!("The sum of i32 {a} and {b} is {}", a + b);

    let a: u64 = 1; // Unsigned 64-bit
    let b: u64 = 2;

    // Basic Operations
    println!("Division (integer): {} / {} = {}", a, b, a / b); // Truncates towards zero
    println!("Remainder (modulo): {} % {} = {}", a, b, a % b);

    // --- Compound Types: Tuples ---
    // Tuples group multiple types together. They have a fixed length.
    let tup: (i32, u64, f64, char) = (500, 3, 34.2, 'c');

    // Accessing via dot notation
    println!("Access by index: tup.0={}", tup.0);

    // Destructuring: breaking the tuple into individual variables
    let (x, y, z, w) = tup;
    println!("Destructured: x={x}, y={y}, z={z}, w={w}");

    // --- Compound Types: Arrays ---
    // Arrays are fixed-size and stored on the STACK.
    // [type; length]
    let ar: [i32; 5] = [2, 3, 4, 5, 6];
    println!("Last element: {}", ar[4]);

    // Rust provides safety: accessing ar[5] would cause a
    // compile-time error or a runtime panic, preventing buffer overflows.

    let days: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
    println!("First day: {}, Last day: {}", days[0], days[6]);

    println!("Please enter an array index: ");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = days[index];
    println!("The value of the element of the index {index} is: {element}")
}
