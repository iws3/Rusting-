# 🔄 Loops in Rust

Rust provides three kinds of loops: `loop`, `while`, and `for`.

## The `loop` Loop

The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

```rust
loop {
    println!("again!");
    break; // Breaks out of the loop
}
```

### Returning Values from Loops

One of the uses of a `loop` is to retry an operation you know might fail, such as checking whether a thread has completed its job. You can add the value you want returned after the `break` expression you use to stop the loop; that value will be returned out of the loop so you can use it, as shown here:

```rust
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

## The `while` Loop

A standard while loop runs as long as a condition is true.

```rust
let mut number = 3;

while number != 0 {
    println!("{number}!");
    number -= 1;
}

println!("LIFTOFF!!!");
```

## The `for` Loop

For loops are the most common loops in Rust because of their safety and conciseness. You can use them to iterate over a collection, such as an array or a range.

```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {element}");
}
```

### Ranges

To execute some code a certain number of times, you can use a `Range`, provided by the standard library, which generates all numbers in sequence starting from one number and ending before another number.

```rust
for number in (1..4).rev() {
    println!("{number}!");
}
println!("LIFTOFF!!!");
```

## Further Reading
- [Rust Book: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
