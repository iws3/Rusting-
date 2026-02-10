# ✏️ Borrowing — Exercises

Write your solutions in:
`students/your_username/part2/02_borrowing/solutions.rs`

---

## Exercise 1 — Fix the Borrow Checker 🔧

The following functions all have borrow checker violations.
For each one: copy it, add a comment explaining the violation,
then fix it using references.

```rust
// Function A: shouldn't need to take ownership
fn get_length(s: String) -> usize {
    s.len()
}

// Function B: has an illegal double mutable borrow
fn double_mut_borrow() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{} and {}", r1, r2);
}

// Function C: mutable + immutable borrow overlap
fn mixed_borrows() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    println!("{}, {}, and {}", r1, r2, r3);
}
```

---

## Exercise 2 — The Library System 📚

Build a simple library catalog using borrowing rules:

```rust
struct Library {
    books: Vec<String>,
}
```

Implement these functions using proper borrowing:

1. `add_book(library: &mut Library, title: String)`
   Adds a book to the library — needs mutable borrow

2. `display_catalog(library: &Library)`
   Prints all books — only needs immutable borrow

3. `find_book(library: &Library, title: &str) -> bool`
   Searches for a book — immutable borrow

4. `count_books(library: &Library) -> usize`
   Returns count — immutable borrow

In `exercise_2()`, create a Library, add 5 books, display the
catalog, search for 2 books (one that exists, one that doesn't),
and print the count. Show that you can call display_catalog and
find_book multiple times because they only borrow immutably.

---

## Exercise 3 — Borrow Scope Analysis 🔬

This exercise is about understanding non-lexical lifetimes —
where Rust is smart about when borrows actually end.

Write `exercise_3()` that demonstrates all of the following
in one function, with comments explaining why each transition
is valid or invalid:

1. Create a mutable String
2. Create two immutable references, use them, then let them go
3. AFTER they've been used (last use = end of borrow), create
   a mutable reference and modify the string
4. After the mutable reference is last used, create new
   immutable references again
5. Print the string at each stage

The goal is to show that Rust's borrow checker works based
on **last use**, not code block scope.
