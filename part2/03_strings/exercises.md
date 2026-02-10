# ✏️ Strings — Exercises

Write your solutions in:
`students/your_username/part2/03_strings/solutions.rs`

---

## Exercise 1 — String Type Explorer 🔭

Write `exercise_1()` demonstrating the differences between
`String` and `&str` concretely:

1. Create a `&str` from a string literal
2. Create an owned `String` from it using two different methods
3. Create a `&str` slice from the owned String
4. Show that `&str` can be created from a `String` via `&` and `&string[..]`
5. Write a function `first_word(s: &str) -> &str` that finds and
   returns the first word (everything before the first space,
   or the whole string if no space). Test it with:
   - `"hello world"`
   - `"rust"`
   - `"  leading space"`
6. Write a function `accepts_both(s: &str)` that prints the string —
   call it with both a `&str` and a `String` to prove both work

---

## Exercise 2 — String Manipulation Toolkit 🛠️

Write `exercise_2()` that processes a passage of text.

Start with this string:
```
let text = String::from("  The quick brown fox jumps over the lazy dog.  ");
```

Apply and print the result of each transformation:
1. `.trim()` — remove leading/trailing whitespace
2. `.to_uppercase()`
3. `.to_lowercase()`
4. `.replace("fox", "🦀")` — replace a word with crab emoji
5. Split by spaces and print each word on its own line with its index
6. Count how many words contain the letter 'o'
7. Check if the text contains the substring "quick"
8. Collect the words into a new String joined by " | " using
   `.split_whitespace().collect::<Vec<&str>>().join(" | ")`

---

## Exercise 3 — UTF-8 Deep Dive 🌍

This exercise explores Rust's Unicode-correct string handling.

Write `exercise_3()` with the following:

1. Create a String containing text from three different scripts:
   ```
   let multilingual = String::from("Hello Привет 你好 🌍");
   ```

2. Print the number of **bytes** in the string using `.len()`
   (this counts raw bytes, not characters)

3. Print the number of **Unicode characters** using `.chars().count()`
   (this counts actual characters)

4. Explain in a comment WHY these two numbers are different

5. Print each character with its index using `.chars().enumerate()`

6. Show that you CANNOT index a string with `s[0]` — write the
   correct way to get the first character using `.chars().next()`

7. Build a new String from individual chars:
   Collect only the ASCII characters (chars where `c.is_ascii()`)
   from the multilingual string into a new String

Print all results with clear labels.
