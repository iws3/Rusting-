# ✏️ Data Types — Exercises

Write your solutions in:
`students/your_username/part1/02_data_types/solutions.rs`

---

## Exercise 1 — Type Detective 🔍

For each value below, write the correct Rust type annotation AND
explain in a comment why you chose that type (bit size, signed
vs unsigned, etc.).

Write a function `exercise_1()` that declares all of these with
explicit type annotations and prints each one:

| Value | Your job |
|-------|----------|
| `255` | Must fit in exactly 1 byte, no negatives |
| `-32000` | Needs to be negative, must fit in 2 bytes |
| `3.14159265358979` | Needs maximum decimal precision |
| `'🦀'` | A single Unicode character |
| `true` | Obvious, but annotate it explicitly |
| `1_000_000_000` | One billion, positive only, 4 bytes |

**Expected output:**
```
255 is a u8
-32000 is an i16
3.14159265358979 is an f64
🦀 is a char
true is a bool
1000000000 is a u32
```

---

## Exercise 2 — Tuple Passport 🛂

Model a passport using a tuple. A passport contains:
- Full name (use a string literal `&str` for now)
- Age (u8 — a person can't be older than 255)
- Country code (a 2-char array: `[char; 2]`)
- Is valid (bool)

Write a function `exercise_2()` that:
1. Creates the passport tuple
2. Destructures it into four separate variables
3. Prints each field with a label
4. Access the age a second time using **dot notation** (not
   the destructured variable) and print it

**Expected output:**
```
Name: Jane Doe
Age: 28
Country: CM
Valid: true
Age via dot notation: 28
```

---

## Exercise 3 — Overflow Awareness 🌊

This exercise teaches you about integer overflow and casting.

Write a function `exercise_3()` that:

1. Declares a `u8` variable with value `250`
2. Prints it
3. Declares a `u16` variable that takes the u8 value and adds
   `1000` to it — you'll need to cast: `value as u16`
4. Prints the u16 result
5. Declares an `i32`, an `f64`, and converts the i32 to f64
   using `as f64`. Show that `7 / 2` in integers equals `3`,
   but `7.0 / 2.0` in floats equals `3.5`
6. Print both results with labels

**Expected output:**
```
u8 value: 250
u16 value after addition: 1250
Integer division 7/2 = 3
Float division 7.0/2.0 = 3.5
```

**Key insight to explain in a comment:**
Why does integer division truncate instead of rounding?
