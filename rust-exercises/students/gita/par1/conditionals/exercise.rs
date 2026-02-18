
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

fn main() {
    println!("HELLO WORLD");
    exercise_1(80);
}