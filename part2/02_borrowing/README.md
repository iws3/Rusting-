# 💳 References and Borrowing

Borrowing helps us use a value without taking ownership.

## References

A reference is like a pointer in that it’s an address we can follow to access the data stored at that address. But unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Pass a reference
}

fn calculate_length(s: &String) -> usize { // Takes a reference
    s.len()
}
```

## Mutable References

You can change data you borrow if you use a mutable reference.

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

## The Rules of References

1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.

## Further Reading
- [Rust Book: References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
