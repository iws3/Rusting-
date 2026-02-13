# 🗝️ Ownership in Rust

Ownership is Rust's most unique feature and has deep implications for the rest of the language.

## The Rules of Ownership

1. Each value in Rust has an *owner*.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

## Variable Scope

```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

## Move ("Shallow Copy" + Invalidation)

For heap-allocated data (like `String`), assigning one variable to another *moves* the ownership.

```rust
let s1 = String::from("hello");
let s2 = s1;

// println!("{}, world!", s1); // Error! s1 is no longer valid
```

## Clone

If we want to deeply copy the heap data of the `String`, not just the stack data, we can use a common method called `clone`.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

## Stack-Only Data: Copy

Types like integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y); // Valid!
```

## Further Reading
- [Rust Book: Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
