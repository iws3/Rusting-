// ## Exercise 2 — Tuple Passport 🛂

// Model a passport using a tuple. A passport contains:
// - Full name (use a string literal `&str` for now)
// - Age (u8 — a person can't be older than 255)
// - Country code (a 2-char array: `[char; 2]`)
// - Is valid (bool)

// Write a function `exercise_2()` that:
// 1. Creates the passport tuple
// 2. Destructures it into four separate variables
// 3. Prints each field with a label
// 4. Access the age a second time using **dot notation** (not
//    the destructured variable) and print it

// **Expected output:**
// ```
// Name: Jane Doe
// Age: 28
// Country: CM
// Valid: true
// Age via dot notation: 28
// ```

fn exercise_2(){
    // create a turple here
    let password_tuple:(i32, i32, i32)=(1,3,4);
    println!("The values of my tuples are :{:?}", password_tuple);

}

fn main(){
    exercise_2();
}