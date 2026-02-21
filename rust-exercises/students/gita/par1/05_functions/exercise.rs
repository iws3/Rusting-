// # ✏️ Functions — Exercises

// Write your solutions in:
// `students/your_username/part1/05_functions/solutions.rs`

// ---

// ## Exercise 1 — Expression vs Statement 🤔

// This exercise is about deeply understanding how Rust functions return
// values through expressions.

// Write these four functions AND explain with a comment what each
// one returns and why:



// **Function 1:** `is_even(n: i32) -> bool`
fn is_even(n:i32)->bool {
    if n%2==0 {
     return true;
    }

    return false;
    
}
// Returns true if n is even using an expression (no `return` keyword)

// **Function 2:** `classify_temperature(temp: f64) -> &'static str`
fn classify_temperature(temp:f64)->& 'static str {
    if temp<0.0 {
        "freezing"
    }
    else if temp <15.0 {
        "cold"
    }
    else if temp<25.0 {
        "hot"
    }
    else{
        "warning"
    }
 
}


fn calculate_bmi(weight_kg:f64, height_m:f64)->f64 {
    weight_kg/(height_m * weight_kg)

}

fn describe_bmi(bmi:f64)->& 'static str {

    if bmi<18.5 {
        "underweight"
    }
    else if bmi>=18.5 && bmi<=25.0 {
        "Normal"
    }
    else if bmi >=25.0 && bmi<=30.0 {
        "overweight"
    }
    else if bmi>30.0 {
        "Obese"
    }
    else {
        "Invalid BMI value"
    }

}

fn exercise_1(){
    is_even(23);
    let temp_classify=classify_temperature(20.7);
    println!("TEMPERATURE CLASS: {}", temp_classify);
    let bmi=calculate_bmi(45.4, 2.4);
    println!("BMI PRINTED IS: {}", bmi);
    let bmi_description=describe_bmi(bmi);
    println!("DESCRIBE BMI: {}", bmi_description);


}

fn main() {
    exercise_1();
}
// Returns "freezing" if < 0.0, "cold" if < 15.0, "warm" if < 25.0,
// else "hot" — using `if` as an expression

// **Function 3:** `calculate_bmi(weight_kg: f64, height_m: f64) -> f64`
// Returns weight / (height * height) as an expression

// **Function 4:** `describe_bmi(bmi: f64) -> &'static str`
// Under 18.5 → "Underweight", 18.5–25.0 → "Normal",
// 25.0–30.0 → "Overweight", above 30.0 → "Obese"

// In `exercise_1()`, call all four with sample values and print results.

// ---