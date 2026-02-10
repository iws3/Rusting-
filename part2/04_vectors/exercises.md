# ✏️ Vectors — Exercises

Write your solutions in:
`students/your_username/part2/04_vectors/solutions.rs`

---

## Exercise 1 — Vec Operations Mastery 💪

Write `exercise_1()` that demonstrates core Vec operations:

1. Create an empty `Vec<i32>` and push numbers 1–10 into it
2. Print the vector with `{:?}`
3. Print the first and last elements safely using `.get()`
4. Remove the last element with `.pop()` and print what was popped
   (pop returns `Option<T>`)
5. Insert the number `99` at index `3` using `.insert()`
6. Remove the element at index `5` using `.remove()`
7. Sort the vector with `.sort()`
8. Reverse it with `.reverse()`
9. Check if it contains `99` using `.contains()`
10. Print the final vector and its length

---

## Exercise 2 — Statistics Calculator 📊

Write a function `calculate_stats(numbers: &Vec<i32>)`
that computes and prints the following for a given vector:
- Minimum value
- Maximum value
- Sum of all values
- Average (as f64)
- Count of even numbers
- Count of odd numbers
- Count of numbers greater than the average

Do NOT use built-in `.min()` or `.max()` methods for min/max —
compute them manually using a loop to practice iteration.

In `exercise_2()`, call your function with:
- `vec![4, 8, 15, 16, 23, 42]`
- A vector of all numbers from 1 to 20

---

## Exercise 3 — The Gradebook 📒

Build a gradebook system using vectors.

```rust
struct Student {
    name: String,
    grades: Vec<f64>,
}
```

Implement these functions:

1. `new_student(name: &str) -> Student`
   Creates a student with an empty grades vector

2. `add_grade(student: &mut Student, grade: f64)`
   Adds a grade to the student's grades vector

3. `average_grade(student: &Student) -> f64`
   Returns the average of all grades (return 0.0 if no grades)

4. `highest_grade(student: &Student) -> f64`
   Returns the highest grade

5. `letter_grade(student: &Student) -> &str`
   Returns letter grade based on average

6. `print_report(student: &Student)`
   Prints a full formatted report

In `exercise_3()`, create 3 students with different grade sets,
add grades to each, then print all reports.
