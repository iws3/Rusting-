fn exercise_1() {
    let mut x = 10;
    x = 20;
    println!("x is {}", x);

    let mut greeting = "hello";
    greeting = "goodbye";
    println!("{}", greeting);

    const MAX_SCORE:u32 = 100;
    println!("Max score is {}", MAX_SCORE);
}

fn main(){
    println!("Hello, World");
    exercise_1();
}