// ============================================
// Student: mntengkeh
// Topic: Vectors (Part 2, Day 13)
// Date: 2026-02-03
// ============================================

// Exercise 1
fn exercise_1() {
    let mut v: Vec<u32> = Vec::new();
		for i in 1..11 {
			v.push(i);
		}
		println!("Initial vector: {:?}", v); 
		println!("First: {}, Last: {}", v.get(0).unwrap(), v.get(v.len()-1).unwrap());
		println!("Last element removed: {}", v.pop().unwrap());
		println!("Insert 99 at index 3");
		v.insert(3, 99);
		println!("{:?}", v);
		println!("Remove element at index 5");
		let x = v.remove(5);
		println!("{:?}", v);
		println!("Sort vector");
		v.sort();
		println!("{:?}", v);
		println!("Reverse vector");
		v.reverse();
		println!("{:?}", v);
		println!("Vector contains 99: {}", v.contains(&99));
		println!("Final Vector: {:?}, Length: {}", v, v.len());
		
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
