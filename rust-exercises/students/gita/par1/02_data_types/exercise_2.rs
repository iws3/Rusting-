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
    let password_tuple:(&str, u8, [char; 2], bool)=("john",33,['w','r'], false);
    let (name, age, char_2,is_valid)=password_tuple;

    println!("The values of my tuples are :{:?}", password_tuple);
    println!("NAME: {}", name);
    println!("AGE: {}", age);
    println!("CHAR 1: {}", char_2[0]);
    println!("ISVALID: {}", is_valid);
    println!("DOT..{}", password_tuple.0);

}

fn exercise_3(){
    // WRITE EXERCISE 3 HERE
}

fn main(){
    exercise_2();
}