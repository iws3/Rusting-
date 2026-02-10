# 📦 Variables in Rust

## What You Need to Know

In Rust, variables are **immutable by default**. This is one of Rust's
most important design decisions. Once you bind a value to a name, that
binding cannot change unless you explicitly allow it with `mut`.

### Immutable Variables
```rust
let x = 5;
// x = 6; // ← This would be a compile error
```

### Mutable Variables
```rust
let mut y = 5;
y = 6; // ← This is fine
```

### Shadowing
You can re-declare a variable with the same name using `let` again.
This is called **shadowing**. It's different from mutation because:
- You can change the type
- The old variable isn't modified — a new one is created

```rust
let x = 5;
let x = x + 1;     // x is now 6
let x = x * 2;     // x is now 12
let x = "now I'm a string!"; // Type changed — shadowing allows this
```

### Constants
Constants are always immutable, must have a type annotation, and
can be declared in any scope including global scope:

```rust
const MAX_POINTS: u32 = 100_000;
```

## Key Differences From JavaScript/Python

| Feature | JavaScript | Python | Rust |
|---------|-----------|--------|------|
| Default mutability | Mutable | Mutable | **Immutable** |
| Type change on reassign | Yes | Yes | Only via shadowing |
| Constants | `const`/`let` | Convention only | `const` enforced |

## Further Reading
- [Rust Book: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
