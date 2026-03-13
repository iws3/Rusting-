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
    // Your solution here
}

// Exercise 3
fn exercise_3() {
    // Your solution here
}

fn main() {
    exercise_1();
    exercise_2();
    exercise_3();
}