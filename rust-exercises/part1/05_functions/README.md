# 🛠️ Functions in Rust

Functions are the building blocks of Rust code.

## Defining a Function

Use the `fn` keyword to declare a function. Rust uses snake case as the conventional style for function and variable names.

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

## Parameters

We can define functions to have parameters, which are special variables that are part of a function's signature.

```rust
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

## Statements and Expressions

- **Statements** are instructions that perform some action and do not return a value.
- **Expressions** evaluate to a resulting value.

```rust
let y = {
    let x = 3;
    x + 1
};
// y is 4
```

## Functions with Return Values

We don't name return values, but we usually declare their type after an arrow (`->`). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly.

```rust
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

## Further Reading
- [Rust Book: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
