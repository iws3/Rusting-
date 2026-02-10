# ✏️ Ownership — Exercises

Write your solutions in:
`students/your_username/part2/01_ownership/solutions.rs`

---

## Exercise 1 — Move Detective 🕵️

Read each code snippet below. For each one, write in a comment:
- Does it compile? Yes or No
- If no, WHY does it fail?
- Fix it WITHOUT using references (use cloning or restructuring)

```rust
// Snippet A
fn snippet_a() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1);
}

// Snippet B
fn snippet_b() {
    let s = String::from("world");
    takes_string(s);
    println!("{}", s);
}
fn takes_string(s: String) {
    println!("{}", s);
}

// Snippet C
fn snippet_c() {
    let x = 42;
    let y = x;
    println!("{}", x); // Does this compile?
}
```

Explain in comments why Snippet C behaves differently from A and B.

---

## Exercise 2 — Ownership Chain 🔗

Write a function chain that passes ownership between functions:

1. `create_greeting() -> String`
   Creates and returns the String `"Hello from Rust!"`

2. `shout_it(s: String) -> String`
   Takes ownership, converts to uppercase using `.to_uppercase()`,
   returns the new owned String

3. `measure_it(s: String) -> (String, usize)`
   Takes ownership, returns a tuple of the String AND its length

4. In `exercise_2()`:
   - Call `create_greeting()` → store in variable
   - Call `shout_it()` with it → store in new variable
   - Call `measure_it()` with it → destructure the tuple
   - Print the final string and its length

Trace in comments where ownership lives at each step.

---

## Exercise 3 — Clone vs Move 🧬

Write `exercise_3()` demonstrating when clone is appropriate
and when it's wasteful.

**Part A:** Show a case where `.clone()` is necessary:
Create a String, pass it to a function, but also print it
after the function call. Use clone so you can do both.

**Part B:** Show a case where clone is wasteful:
You have a String and you want to find its length. Instead
of cloning to pass it to a function, redesign the function
to not need ownership (for now, just pass the String in and
return it back as a tuple — no references yet).

**Part C:** Show what happens with Copy types:
Demonstrate that `i32`, `f64`, `bool`, and `char` all
automatically copy when assigned, so you never need `.clone()`.

For each part, print output that proves your code works correctly
and add comments explaining your ownership decisions.
