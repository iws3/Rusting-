// ============================================
// Student: mntengkeh
// Topic: Loops (Part 1, Day 3)
// Date: 2026-02-17
// ============================================

// Exercise 1
fn exercise_1() {
    for i in 1..=30 {
			if (i % 3 == 0) && (i % 5 == 0) {
				println!("FizzBuzz");
			} else if i % 3 == 0 {
				println!("Fizz");
			} else if i % 5 == 0 {
				println!("Buzz");
			} else {
				println!("{}", i);
			}
		}
}

// Exercise 2
fn exercise_2() {

}

// Exercise 3
fn exercise_3() {
		println!("Multiplication Table\n");
		for i in 1..6 {
			for j in 1..6 {
				println!("{} x {} = {}", i, j, i * j);
			}
			println!("\n");
		}
		
}

fn main() {
    //exercise_1();
    exercise_2();
    exercise_3();
}
