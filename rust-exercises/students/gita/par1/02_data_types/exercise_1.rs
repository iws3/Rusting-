// ## Exercise 1 — Type Detective 🔍

// For each value below, write the correct Rust type annotation AND
// explain in a comment why you chose that type (bit size, signed
// vs unsigned, etc.).

// Write a function `exercise_1()` that declares all of these with
// explicit type annotations and prints each one:

// | Value | Your job |
// |-------|----------|
// | `255` | Must fit in exactly 1 byte, no negatives |
// | `-32000` | Needs to be negative, must fit in 2 bytes |
// | `3.14159265358979` | Needs maximum decimal precision |
// | `'🦀'` | A single Unicode character |
// | `true` | Obvious, but annotate it explicitly |
// | `1_000_000_000` | One billion, positive only, 4 bytes |

// **Expected output:**
// ```
// 255 is a u8
// -32000 is an i16
// 3.14159265358979 is an f64
// 🦀 is a char
// true is a bool
// 1000000000 is a u32
// ```

fn exercise_1() {
    // unsign integer, store values up to 255
    let value1: u8 = 255;
    // sign integers, store negative and positive values
    let value2: i16 = -32000;
    // store floating point values
    let value3: f64 = 3.14159265358979;
    let value4: char = '🦀';
    let value5: bool = true;
    let value6: u32 = 1000000000;
    println!("The value of value1: {}", value1);
    println!("The value of value2: {}", value2);
    println!("The value of value3: {}", value3);
    println!("The value of value4: {}", value4);
    println!("The value of value5: {}", value5);
    println!("The value of value6: {}", value6);
}
fn main(){
    exercise_1();
}
