# 📦 Vectors in Rust

`Vec<T>` is a resizable array. It stores values of the same type next to each other in memory.

## Creating a Vector

```rust
let v: Vec<i32> = Vec::new();
let v2 = vec![1, 2, 3]; // Using the macro
```

## Updating a Vector

```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

## Reading Elements

You can access elements via indexing or the `get` method.

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2]; // Can panic if out of bounds
println!("The third element is {third}");

match v.get(2) { // Returns Option<&T>, safe
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

## Iterating

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}
```

## Further Reading
- [Rust Book: Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
