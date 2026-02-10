# ✏️ Loops — Exercises

Write your solutions in:
`students/your_username/part1/03_loops/solutions.rs`

---

## Exercise 1 — FizzBuzz With a Twist 🎯

The classic. But in Rust — using a `for` loop with a range.

Write `exercise_1()` that loops from `1` to `30` (inclusive) and:
- If the number is divisible by 3 AND 5 → print `FizzBuzz`
- If divisible by 3 only → print `Fizz`
- If divisible by 5 only → print `Buzz`
- Otherwise → print the number

**Hint:** The modulo operator is `%`. `n % 3 == 0` means divisible by 3.

---

## Exercise 2 — Loop with a Return Value 💰

Use a `loop` (not `while` or `for`) to find the first number
greater than `1` that is both:
- A perfect square (e.g., 4, 9, 16, 25...)
- Divisible by 7

Write `exercise_2()` that:
1. Uses a `loop` (which is an expression) that increments a counter starting at `2`
2. Checks the conditions
3. When found, **breaks with the value** (remember: `break value;`)
4. Stores the result from the loop
5. Prints it

**Expected output:**
```
First perfect square divisible by 7: 196
```

**Hint:** To check if a number is a perfect square, compute its
square root with `(n as f64).sqrt()` and check if the result has
no decimal part: `sqrt == sqrt.floor()`

---

## Exercise 3 — Multiplication Table 📊

Write `exercise_3()` that prints a formatted multiplication table
for numbers 1 through 5 using **nested for loops**.

**Expected output:**
```
1  x  1 =  1
1  x  2 =  2
1  x  3 =  3
1  x  4 =  4
1  x  5 =  5
2  x  1 =  2
...
5  x  5 = 25
```

**Bonus challenge:** Use `{:>2}` in your format string to right-align
numbers in a field of width 2, making the table perfectly aligned.
