# 🔢 Data Types in Rust

## The Scalar Types

Rust has four scalar (single value) types.

### Integers

| Type | Bits | Min | Max |
|------|------|-----|-----|
| i8 | 8 | -128 | 127 |
| i16 | 16 | -32,768 | 32,767 |
| i32 | 32 | -2,147,483,648 | 2,147,483,647 |
| i64 | 64 | -9.2 × 10¹⁸ | 9.2 × 10¹⁸ |
| u8 | 8 | 0 | 255 |
| u16 | 16 | 0 | 65,535 |
| u32 | 32 | 0 | 4,294,967,295 |
| u64 | 64 | 0 | 1.8 × 10¹⁹ |
| isize | arch | depends | depends |
| usize | arch | 0 | depends |

**The `i` prefix means signed (negative + positive).**
**The `u` prefix means unsigned (positive only).**
**The number is the bit width.**

### Floats
```rust
let f32_val: f32 = 3.14;  // 32-bit, ~7 decimal digits precision
let f64_val: f64 = 3.14;  // 64-bit, ~15 decimal digits precision (default)
```

### Boolean
```rust
let t: bool = true;
let f: bool = false;
// Takes 1 byte in memory (not 1 bit, for CPU alignment reasons)
```

### Character
```rust
let c: char = 'z';
let emoji: char = '😊';
// Always 4 bytes — represents any Unicode Scalar Value
```

## Compound Types

### Tuple — fixed size, mixed types
```rust
let tup: (i32, f64, bool) = (500, 6.4, true);
let (x, y, z) = tup;       // destructuring
let first = tup.0;          // dot notation access
```

### Array — fixed size, same type
```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 10]; // ten zeros
let first = arr[0];
```

## Type Inference vs Type Annotation

Rust can infer most types, but sometimes you must annotate:
```rust
let inferred = 42;          // Rust infers i32
let annotated: i64 = 42;    // You tell Rust it's i64
let parsed = "42".parse::<i32>().unwrap(); // Must specify type
```
