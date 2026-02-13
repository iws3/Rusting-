# 🚦 Enums in Rust

Enums (enumerations) allow you to define a type by enumerating its possible variants.

## Defining an Enum

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

## Enum Values

We can create instances of each of the two variants of `IpAddrKind`.

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

## Enums with Data

Rust enums are more powerful than in C or C++; each variant can optionally have data associated with it.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

## The `Option` Enum

Rust doesn't have null, but it has an enum that can encode the concept of a value being present or absent.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

## Further Reading
- [Rust Book: Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
