// ============================================
// Student: mntengkeh
// Topic: Structs (Part 2, Day 16)
// Date: 2026-03-08
// ============================================

// Exercise 1
fn exercise_1() {
    let mut account1 = BankAccount::new(String::from("First User"), 1001);
    let mut account2 = BankAccount::new(String::from("Second User"), 1002);

    account1.deposit(5000.0);
    account1.withdraw(3000.0);
    account1.print_statement();
    // account1.close()
    account2.deposit(7000.0);
    account2.withdraw(5000.0);
    account2.print_statement();
    account2.close();
    println!("{:?}", account2.deposit(700.0));
    account2.print_statement();
}

#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: f64,
    account_number: u32,
    is_active: bool,
}

impl BankAccount {
    fn new(owner: String, account_number: u32) -> Self {
        Self {
            owner,
            account_number,
            balance: 0.0,
            is_active: true
        }
    }

    fn deposit(&mut self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 { 
            return Err(String::from("Invalid amount")); 
        }
        if  !self.is_active {
            return Err(String::from("Account inactive!")); 
        }
        self.balance += amount;
        return Ok(self.balance);
    }

    fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount > self.balance { 
            return Err(String::from("Invalid amount")); 
        }
        if  !self.is_active {
            return Err(String::from("Account inactive!")); 
        }
        self.balance -= amount;
        return Ok(self.balance);
    }

    fn close(&mut self) {
        self.is_active = false;
    }

    fn print_statement(&self) {
        println!("\nBank Summary\n");
        println!("{:#?}", self);
    }


}


// Exercise 2
fn exercise_2() {
    let vector: Vector2D = Vector2D::new(3.0, 4.0);
    let vector1: Vector2D = Vector2D::new(5.0, 8.0);
    println!("\nVectorA: {:?}", vector);
    println!("VectorB: {:?}", vector1);

    println!("\nVectorA magnitude: {}", vector.magnitude());
    print!("\nVectorA + VectorB: ");
    vector.add(&vector1).print();

    print!("\nVectorA - VectorB: ");
    vector.subtract(&vector1).print();

    println!("\nVectorA dot VectorB: {}", vector.dot_product(&vector1));

    print!("\nScale VectorA by 5: ");
    vector.scale(5.0).print();

    print!("\nNormalized VectorA: ");
    vector.normalize().print();

}

#[derive(Debug)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y
        }
    }

    fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }

    fn add(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

    fn subtract(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }

    fn dot_product(&self, other: &Vector2D) -> f64 {
        self.x * other.x + self.y * other.y

    } 

    fn scale(&self, factor: f64) -> Vector2D {
        Vector2D {
            x: self.x * factor,
            y: self.y * factor
        }
    }

    fn normalize(&self) -> Vector2D {
        Vector2D {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude()
        }
    }

    fn print(&self) {
        println!("{:?}", self);
    }

}

// Exercise 3
fn exercise_3() {
    let query1 = QueryBuilder::new("users")
        .select("name")
        .select("email")
        .where_clause("age > 18")
        .order("name")
        .limit(10)
        .build();

    let query2 = QueryBuilder::new("students")
        .select("name")
        .where_clause("age > 18")
        .where_clause("average < 10")
        .order("name")
        .build();

    let query3 = QueryBuilder::new("vehicle")
        .select("name")
        .select("brand")
        .select("version")
        .where_clause("price > $40_000")
        .limit(7)
        .build();

    println!("{}", query1);  
    println!("{}", query2);  
    println!("{}", query3);  


}

struct QueryBuilder {
    table: String,
    conditions: Vec<String>,
    columns: Vec<String>,
    limit: Option<u32>,
    order_by: Option<String>,
}

impl QueryBuilder {
    fn new(table: &str) -> QueryBuilder {
        QueryBuilder {
            table: table.to_string(),
            conditions: Vec::new(),
            columns: Vec::new(),
            limit: None,
            order_by: None
        }
    }

    fn select(mut self, column: &str) -> QueryBuilder {
        self.columns.push(column.to_string());
        self
    }

    fn where_clause(mut self, condition: &str) -> QueryBuilder {
        self.conditions.push(condition.to_string());
        self
    }

    fn limit(mut self, n: u32) -> QueryBuilder {
        self.limit = Some(n);
        self
    }

    fn order(mut self, column: &str) -> QueryBuilder {
        self.order_by = Some(column.to_string());
        self
    }

    fn build(self) -> String {
        let mut query = String::from("SELECT ");
        for i in 0..self.columns.len() {
            if i == self.columns.len() - 1 {
                query.push_str(&self.columns[i]);
            } else {
                query.push_str(&self.columns[i]);
                query.push_str(", ")
            }
        }

        query.push_str(" FROM ");
        query.push_str(&self.table);
        query.push_str(" WHERE ");

        for i in 0..self.conditions.len() {
            if i == self.conditions.len() - 1 {
                query.push_str(&self.conditions[i]);
            } else {
                query.push_str(&self.conditions[i]);
                query.push_str(" && ");
            }
        }

        query.push_str(" ORDER BY ");

        if let Some(value) = self.order_by {
            query.push_str(&value);
        }

        query.push_str(" LIMIT ");

        if let Some(value) = self.limit {
            query.push_str(&value.to_string());
        }
        
        query
    }
}

fn main() {
    //exercise_1();
    //exercise_2();
    exercise_3();
}