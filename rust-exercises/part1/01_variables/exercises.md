# ✏️ Variables — Exercises

Read the README.md in this folder before starting.
Write your solutions in:
`students/your_username/part1/01_variables/solutions.rs`

---

## Exercise 1 — The Immutability Bug Hunt 🐛

The following code has bugs. Your job is to:

1. Copy this code into your solutions file
2. Fix all the bugs so it compiles and runs correctly
3. Add a comment above each fix explaining WHY the original was wrong
<!-- resring push -->
**Buggy code to fix:**
```rust
fn exercise_1() {
    let x = 10;
    x = 20;
    println!("x is {}", x);

    let greeting = "hello";
    greeting = "goodbye";
    println!("{}", greeting);

    const max_score = 100;
    println!("Max score is {}", max_score);
}
```

**Expected output:**
```
x is 20
goodbye
Max score is 100
```

---

## Exercise 2 — Shadow Boxing 🥊

Use **shadowing** (not mutation) to transform the variable `value`
through four stages. You must use `let value = ...` each time.

Write a function `exercise_2()` that:

1. Starts with `value` as the integer `2`
2. Shadows it to multiply by `10` → should be `20`
3. Shadows it to add `5` → should be `25`
4. Shadows it to convert to a String using `.to_string()` → `"25"`
5. Shadows it to add the string `" is the answer"` using `format!()`
6. Prints the final value

**Expected output:**
```
25 is the answer
```

**Hint:** `format!("{}{}", a, b)` concatenates two strings.

---

## Exercise 3 — The Temperature Vault 🌡️

Write a function `exercise_3()` that:

1. Declares a constant `ABSOLUTE_ZERO_C: f64` equal to `-273.15`
2. Declares a mutable variable `temperature` starting at `100.0` (f64)
3. Prints the starting temperature
4. Changes temperature to `-10.5`
5. Prints the new temperature
6. Prints absolute zero from the constant
7. Calculates and prints the difference between temperature and
   absolute zero (temperature - ABSOLUTE_ZERO_C)

**Expected output:**
```
Starting temperature: 100°C
Current temperature: -10.5°C
Absolute zero: -273.15°C
Difference from absolute zero: 262.65°C
```

**Hint:** Use `println!("{:.2}", value)` to print 2 decimal places.
