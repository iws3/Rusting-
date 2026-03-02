
// ## Exercise 1 — Grade Calculator 📝

// Write `exercise_1()` that takes a score as a `u32` and uses a
// chain of `if / else if / else` to determine a grade.
// Since we haven't covered functions with parameters yet, hardcode
// three different scores and call your grading logic three times:
// `95`, `72`, and `45`.

// Rules:
// - 90–100 → "A — Excellent!"
// - 80–89  → "B — Good"
// - 70–79  → "C — Average"
// - 60–69  → "D — Below Average"
// - 0–59   → "F — Failing"

// **Expected output:**
// ```
// 95: A — Excellent!
// 72: C — Average
// 45: F — Failing
// ```

fn exercise_1(score:u32) {

    if score >= 90 && score <=100 {
        println!("YOUR GARDE -A");
    }
    else if score >= 80 && score<=89 {
        println!("YOUR GRADE - B");
    }
    else if score >= 70 && score <=79 {
        println!("YOUR GRADE - C");
    }
    else if score > 60 && score <=69 {
        println!("YOUR GRADE -D");
    }
    else if score <=59 {
        println!("FAILING.......");
    }
    else {
        println!("OUT OF RANGE");
    }

}



// ## Exercise 2 — if as an Expression 🧮

// Rust's `if` returns a value. Use this feature throughout.

// Write `exercise_2()` that:
// 1. Takes a number `n = 17`
// 2. Uses `if` as an expression to assign `"odd"` or `"even"` to a variable
// 3. Uses a nested `if` expression to classify the number:
//    - Less than 10 → "small"
//    - 10–99 → "medium"
//    - 100+ → "large"
// 4. Uses an `if` expression to compute `absolute_value` from `-42`
//    without using the built-in `.abs()` method
// 5. Prints all results

// **Expected output:**
// ```
// 17 is odd
// 17 is medium
// Absolute value of -42 is 42
// ```

fn exercise_2() {
    // 1. Takes a number n = 17
    let n = 17;

    // 2. Uses `if` as an expression to assign "odd" or "even"
    let parity = if n % 2 == 0 { "even" } else { "odd" };

    // 3. Uses a nested `if` expression to classify the number
    let size = if n < 10 {
        "small"
    } else if n <= 99 {
        "medium"
    } else {
        "large"
    };

    // 4. Uses an `if` expression to compute absolute value of -42
    let number = -42;
    let absolute_value = if number < 0 { -number } else { number };

    // 5. Prints all results
    println!("{} is {}", n, parity);
    println!("{} is {}", n, size);
    println!("Absolute value of {} is {}", number, absolute_value);
}


// ## Exercise 3 — Traffic Light Simulator 🚦

// Write `exercise_3()` that simulates a traffic light system.

fn exercise_3() {

    let light:&str="red";
    if light=="red" {
        println!("Stop!"); 
        println!("RED ->30s")    
    }
    else if light=="green" {
        println!("go!");
        println!("GREEN->25s");
    }

    else if light=="yellow" {
        println!("CAUTION");
        println!("yellow->5s");
    }

    else {
        println!("No match yet");
    }

}

// Define a light state as a `&str` with one of three values:
// `"red"`, `"yellow"`, or `"green"`.

// Using only `if / else if / else` (no match yet!), write logic that:
// 1. Prints what the light means: "Stop", "Caution", or "Go"
// 2. Prints how many seconds the light typically lasts:
//    Red → 30s, Yellow → 5s, Green → 25s
// 3. Prints whether it's safe to cross: Red/Yellow → No, Green → Yes

// Test your logic with all three states by calling it three times.

// **Expected output:**
// ```
// Red light: Stop | Duration: 30s | Safe to cross: No
// Yellow light: Caution | Duration: 5s | Safe to cross: No
// Green light: Go | Duration: 25s | Safe to cross: Yes




fn main() {
    exercise_3();
}
