// ## Exercise 2 — Loop with a Return Value 💰

// Use a `loop` (not `while` or `for`) to find the first number
// greater than `1` that is both:
// - A perfect square (e.g., 4, 9, 16, 25...)
// - Divisible by 7

fn exercise_2(mut n: u64)  {

    let mut perfectSqare=[i32, 45];

    loop {
        n = n - 1;
        let m = (n as f64).sqrt() as u64;
        println!("The square root of {} is {}", n, m);
        let squared_m=m*m;
        if n > 1 && squared_m ==n {
            println!("n is perfect square and the value of n is : {}", n);
        }
        else if  n>1 && squared_m!=n{
            println!("Not a perfect sqaure___");
        }
        else {
            break;
        }
        
    }
}

fn main() {
    exercise_2(65);
   
}
// Write `exercise_2()` that:
// 1. Uses a `loop` (which is an expression) that increments a counter starting at `2`
// 2. Checks the conditions
// 3. When found, **breaks with the value** (remember: `break value;`)
// 4. Stores the result from the loop
// 5. Prints it

// **Expected output:**
// ```
// First perfect square divisible by 7: 196
// ```
