# 🧵 Strings in Rust

Strings in Rust are deeper than in many other languages.

## The Two Main String Types

### `String` (Owned)

- Stores data on the heap.
- Is growable (mutable).
- You own it.
- Created with `String::from("hello")` or `"hello".to_string()`.

### `&str` (String Slice / Reference)

- A view into a string.
- Usually points to data owned by a `String` or static data owned by the binary (string literals).
- Immutable view.
- Fast and cheap to pass around.

## UTF-8 Encoding

Rust strings are always valid UTF-8. This means they support emojis and international characters natively, but it also means you cannot index into them like `s[0]` because one character might be more than one byte.

```rust
let hello = "Здравствуйте";
// let answer = &hello[0]; // Error!
```

To iterate over characters, use the `.chars()` method.

## Common Methods

- `push_str()`: Append string slice
- `replace()`: Replace substring
- `trim()`: Remove whitespace
- `split()`: Split string by pattern

## Further Reading
- [Rust Book: Storing UTF-8 Encoded Text with Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
