// The Option Enum: Representing Absence
// Rust doesn't have null. Instead, it has the Option enum, which is built into the standard library and available everywhere:


enum Option<T> {
    Some(T),
    None,
}


// Don't worry about the <T> syntax for now—that's generics, which we'll cover in Part 3. For now, understand that Option either holds Some value or None.
// Why is this better than null? Because the compiler forces you to handle both cases. You can't accidentally use a None value as if it were Some value. Let me show you:



// Pattern Matching: The match Expression
// Pattern matching is how you work with enums. The match expression lets you compare a value against patterns and execute code based on which pattern matches:


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Dime;
    println!("Value: {}", value_in_cents(coin));
}





// Patterns can bind to values inside enum variants:



#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(coin);
}