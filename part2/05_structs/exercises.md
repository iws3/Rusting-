# ✏️ Structs — Exercises

Write your solutions in:
`students/your_username/part2/05_structs/solutions.rs`

---

## Exercise 1 — Bank Account 🏦

Design and implement a `BankAccount` struct:

```rust
struct BankAccount {
    owner: String,
    balance: f64,
    account_number: u32,
    is_active: bool,
}
```

Implement these methods:

1. `BankAccount::new(owner: String, account_number: u32) -> BankAccount`
   Creates account with 0.0 balance, active

2. `deposit(&mut self, amount: f64) -> Result<f64, String>`
   Adds amount to balance. Returns `Err` if amount <= 0.0 or
   account is inactive. Returns `Ok(new_balance)` on success.
   Use this simplified Result for now:
   `if amount <= 0.0 { return Err(String::from("Invalid amount")); }`

3. `withdraw(&mut self, amount: f64) -> Result<f64, String>`
   Deducts amount. Returns Err if insufficient funds or inactive.

4. `close(&mut self)` — sets is_active to false

5. `print_statement(&self)` — prints a formatted account statement

In `exercise_1()`, create 2 accounts, perform several deposits and
withdrawals, attempt an invalid withdrawal, close one account, and
print statements for both.

---

## Exercise 2 — Vector Math 📐

Build a `Vector2D` struct representing a 2D mathematical vector:

```rust
struct Vector2D {
    x: f64,
    y: f64,
}
```

Implement:
1. `Vector2D::new(x: f64, y: f64) -> Vector2D`
2. `magnitude(&self) -> f64` — `sqrt(x² + y²)` using `f64::sqrt()`
3. `add(&self, other: &Vector2D) -> Vector2D` — vector addition
4. `subtract(&self, other: &Vector2D) -> Vector2D`
5. `dot_product(&self, other: &Vector2D) -> f64` — `x1*x2 + y1*y2`
6. `scale(&self, factor: f64) -> Vector2D` — multiply both by factor
7. `normalize(&self) -> Vector2D` — divide both by magnitude
8. `print(&self)` — prints `Vector2D(x: 3.0, y: 4.0)`

In `exercise_2()`, demonstrate each operation with clear output.

---

## Exercise 3 — Builder Pattern 🏗️

Implement a `QueryBuilder` struct that builds a SQL-like query
string using method chaining (the builder pattern):

```rust
struct QueryBuilder {
    table: String,
    conditions: Vec<String>,
    columns: Vec<String>,
    limit: Option<u32>,
    order_by: Option<String>,
}
```

Each method should take `self` (ownership), modify it, and
**return self** so calls can be chained:

1. `QueryBuilder::new(table: &str) -> QueryBuilder`
2. `select(mut self, column: &str) -> QueryBuilder`
3. `where_clause(mut self, condition: &str) -> QueryBuilder`
4. `limit(mut self, n: u32) -> QueryBuilder`
5. `order(mut self, column: &str) -> QueryBuilder`
6. `build(&self) -> String` — constructs the query string

In `exercise_3()`, build and print three different queries:
```rust
let query = QueryBuilder::new("users")
    .select("name")
    .select("email")
    .where_clause("age > 18")
    .order("name")
    .limit(10)
    .build();
println!("{}", query);
// SELECT name, email FROM users WHERE age > 18 ORDER BY name LIMIT 10
```
