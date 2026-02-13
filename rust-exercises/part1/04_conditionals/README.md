# 🔀 Conditionals in Rust

## `if` Expressions

The most common way to handle conditions is the `if` expression.

```rust
let number = 3;

if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```

### Handling Multiple Conditions with `else if`

```rust
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else {
    println!("number is not divisible by 4, 3, or 2");
}
```

### Using `if` in a `let` Statement

Because `if` is an expression, we can use it on the right side of a `let` statement to assign the outcome to a variable.

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
// number will be 5
```

**Note:** The results from each arm of the `if` must be the same type.

```rust
// This will mismatch types and fail to compile:
// let number = if condition { 5 } else { "six" };
```

## Further Reading
- [Rust Book: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
