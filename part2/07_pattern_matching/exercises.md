# ✏️ Pattern Matching — Exercises

Write your solutions in:
`students/your_username/part2/07_pattern_matching/solutions.rs`

---

## Exercise 1 — Match Exhaustiveness 🎯

Write `exercise_1()` demonstrating match exhaustiveness.

**Part A:** Write a function `describe_number(n: i32) -> &'static str`
that uses match with ranges and guards:
- Negative numbers → "negative"
- 0 → "zero"
- 1..=9 → "single digit"
- 10..=99 → "double digit"
- 100..=999 → "triple digit"
- 1000.. → "very large"
- Use a guard for even numbers: add `if n % 2 == 0` to relevant arms

**Part B:** Write a function `parse_command(input: &str)` that matches
a command string and prints what it does:
- "quit" | "exit" | "q" → "Quitting program"
- "help" | "h" | "?" → "Showing help"
- "clear" → "Clearing screen"
- s if s.starts_with("load ") → "Loading file: [rest of string]"
- _ → "Unknown command"

In `exercise_1()`, test both functions with at least 5 inputs each.

---

## Exercise 2 — Destructuring Patterns 📦

Write `exercise_2()` demonstrating destructuring in match:

**Tuples:**
```rust
let points = [(0, 0), (1, 0), (0, 1), (3, 4), (-1, -1)];
```
Match each point and classify:
- (0, 0) → "origin"
- (x, 0) → "on x-axis at x"
- (0, y) → "on y-axis at y"
- (x, y) if x == y → "on diagonal at x"
- (x, y) → "at position (x, y)"

**Structs:**
```rust
struct Color { r: u8, g: u8, b: u8 }
```
Write `classify_color(c: &Color) -> &str` using struct destructuring
in match arms to identify: pure red, pure green, pure blue, white,
black, grey (r==g==b), or "mixed".

**Nested enums:**
```rust
enum Notification {
    Email { from: String, subject: String },
    SMS { from: String, message: String },
    Push { app: String, title: String },
}
```
Write `handle_notification(n: &Notification)` that destructures
and prints the fields of each variant.

---

## Exercise 3 — if let and while let 🔄

Write `exercise_3()` using `if let` and `while let`.

**Part A — if let chains:**
Given a vector of `Option<i32>` values:
```rust
let values: Vec<Option<i32>> = vec![
    Some(1), None, Some(3), None, Some(5), Some(6)
];
```
Use a for loop with `if let` to:
- Sum all Some values
- Count None values
- Print each Some value that is even
- Print the first Some value greater than 4, then break

**Part B — while let:**
Create a Vec and use `.pop()` in a `while let` loop to
process elements in reverse order, printing each.

**Part C — Combining with else:**
Create a function `safe_parse(input: &str) -> Option<i32>`
that returns Some(n) if the string parses, else None.
Use `if let` with `else` to handle both cases cleanly
for the inputs: `"42"`, `"hello"`, `"100"`, `"rust"`.
