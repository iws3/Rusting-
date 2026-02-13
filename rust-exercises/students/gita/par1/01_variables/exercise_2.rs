// Write a function `exercise_2()` that:

// 1. Starts with `value` as the integer `2`
// 2. Shadows it to multiply by `10` → should be `20`
// 3. Shadows it to add `5` → should be `25`
// 4. Shadows it to convert to a String using `.to_string()` → `"25"`
// 5. Shadows it to add the string `" is the answer"` using `format!()`
// 6. Prints the final value

// **Expected output:**
// ```
// 25 is the answer
// ```

// **Hint:** `format!("{}{}", a, b)` concatenates two strings.

fn exercise_2() {
    let x = 2;
    let x = x * 10;
    let x = x + 5;
    let x = x.to_string();
    let x = format!("{} is the answer", x);
    println!("the answer is: {}", x);
}

fn main(){
    exercise_2();
}