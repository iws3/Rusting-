# ✏️ Enums — Exercises

Write your solutions in:
`students/your_username/part2/06_enums/solutions.rs`

---

## Exercise 1 — Shape Calculator 📐

Define an enum `Shape` with variants that carry data:

```rust
enum Shape {
    Circle(f64),              // radius
    Rectangle(f64, f64),      // width, height
    Triangle(f64, f64, f64),  // three sides
    Square(f64),              // side length
}
```

Implement a function `area(shape: &Shape) -> f64` using match:
- Circle: π × r²  (use `std::f64::consts::PI`)
- Rectangle: width × height
- Triangle: Heron's formula:
  `s = (a+b+c)/2`, `area = sqrt(s*(s-a)*(s-b)*(s-c))`
- Square: side²

Implement `perimeter(shape: &Shape) -> f64` similarly.

Implement `describe(shape: &Shape) -> String` that returns
a string like `"Circle with radius 5.0"`.

In `exercise_1()`, create one of each shape and print area,
perimeter, and description.

---

## Exercise 2 — Result-Like Error Handling 🚦

Before using Rust's built-in Result, build your own:

```rust
enum MathResult {
    Success(f64),
    DivisionByZero,
    NegativeSquareRoot(f64),
    Overflow,
}
```

Implement these functions returning MathResult:

1. `safe_divide(a: f64, b: f64) -> MathResult`
2. `safe_sqrt(n: f64) -> MathResult` — Error if n < 0
3. `safe_power(base: f64, exp: u32) -> MathResult`
   — Error if result would exceed f64::MAX (check with `f64::INFINITY`)

Write a `print_result(result: MathResult)` function that
uses match to print either the value or a descriptive error.

In `exercise_2()`, test each function with valid and invalid inputs.

---

## Exercise 3 — State Machine 🤖

Build a vending machine state machine using enums:

```rust
enum VendingState {
    Idle,
    HasMoney(f64),      // amount inserted
    Dispensing(String), // item being dispensed
    OutOfStock(String), // item that's out of stock
    Error(String),      // error message
}
```

```rust
struct VendingMachine {
    state: VendingState,
    inventory: Vec<(String, f64, u32)>, // (name, price, quantity)
    total_earned: f64,
}
```

Implement:
1. `VendingMachine::new() -> VendingMachine` — load with 3 items
2. `insert_money(&mut self, amount: f64)` — transitions to HasMoney
3. `select_item(&mut self, item_name: &str)` — check state and
   inventory, transition to Dispensing or OutOfStock or Error
4. `collect_item(&mut self)` — transition back to Idle
5. `print_status(&self)` — prints current state clearly

In `exercise_3()`, simulate a full vending machine interaction:
insert money, select an item, collect it. Then try selecting an
out-of-stock item and an item you can't afford.
