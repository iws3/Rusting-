# ✏️ Functions — Exercises

Write your solutions in:
`students/your_username/part1/05_functions/solutions.rs`

---

## Exercise 1 — Expression vs Statement 🤔

This exercise is about deeply understanding how Rust functions return
values through expressions.

Write these four functions AND explain with a comment what each
one returns and why:

**Function 1:** `is_even(n: i32) -> bool`
Returns true if n is even using an expression (no `return` keyword)

**Function 2:** `classify_temperature(temp: f64) -> &'static str`
Returns "freezing" if < 0.0, "cold" if < 15.0, "warm" if < 25.0,
else "hot" — using `if` as an expression

**Function 3:** `calculate_bmi(weight_kg: f64, height_m: f64) -> f64`
Returns weight / (height * height) as an expression

**Function 4:** `describe_bmi(bmi: f64) -> &'static str`
Under 18.5 → "Underweight", 18.5–25.0 → "Normal",
25.0–30.0 → "Overweight", above 30.0 → "Obese"

In `exercise_1()`, call all four with sample values and print results.

---

## Exercise 2 — Recursive Thinking 🌀

Write `exercise_2()` that calls two recursive functions:

**Function:** `factorial(n: u64) -> u64`
Computes n! recursively.
- Base case: `factorial(0) = 1`
- Recursive case: `n * factorial(n - 1)`

**Function:** `fibonacci(n: u32) -> u64`
Computes the nth Fibonacci number.
- Base cases: `fibonacci(0) = 0`, `fibonacci(1) = 1`
- Recursive case: `fibonacci(n-1) + fibonacci(n-2)`

In `exercise_2()`, print:
- Factorials of 0 through 10
- Fibonacci numbers 0 through 10

**Expected output (partial):**
```
0! = 1
1! = 1
2! = 2
...
10! = 3628800
fib(0) = 0
fib(1) = 1
...
fib(10) = 55
```

---

## Exercise 3 — The Block Expression Deep Dive 🧱

This exercise makes you use block expressions as values everywhere.

Write `exercise_3()` using these rules:
- You may NOT use the `return` keyword anywhere
- Every value must come from a block expression or an expression

Compute and print the following using block expressions assigned
to variables:

1. `celsius_to_fahrenheit`: convert 100.0°C using the formula
   `(c * 9.0/5.0) + 32.0` — assign the result of a block to the var

2. `string_stats`: given the string `"Hello, Rustaceans!"`, use a
   block that computes and returns a tuple of:
   `(length, uppercase_version)` using `.len()` and `.to_uppercase()`

3. `collatz_steps`: starting from `27`, count how many steps it
   takes to reach `1` using the Collatz conjecture inside a block:
   - If n is even: n = n / 2
   - If n is odd: n = 3 * n + 1
   - Count each step

Print all three results.
