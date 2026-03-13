// ============================================
// Student: mntengkeh
// Topic: Vectors (Part 2, Day 13)
// Date: 2026-02-03
// ============================================

#![allow(unused)]

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
    calculate_stats(&vec![4, 8, 15, 16, 23, 42]);
	let mut sample: Vec<i32> = Vec::new();
	for i in 1..21 {
		sample.push(i);
	}
	calculate_stats(&sample);
}

fn calculate_stats(numbers: &Vec<i32>) {
	let mut min = *numbers.iter().next().unwrap();
	let mut max = *numbers.iter().next().unwrap();
	let mut sum = 0;
	let mut odd_count = 0;
	let mut even_count = 0;
	let mut gt_average_count = 0;

	for i in 0..numbers.len() {
		let x = numbers[i];
		if x < min {
			min = x;
			continue;
		}
		if x > max {
			max = x;
		}

		sum += x;

		//let (a, b) = even_odd_count(i, odd_count, even_count);
		//if a > odd_count {
		// 	odd_count = a;
		// } else {
		// 	even_count = b;
		// }

	}
	println!("\nMinimum value: {}\nMaximum value: {}", min, max);
	println!("Sum of values: {}", sum);

	let average: f64 = sum as f64 / numbers.len() as f64;

	println!("Average of values: {:.2}", average);
	//println!("Odd count: {}\nEven count: {}", odd_count, even_count);

	for i in numbers.iter() {
		if *i & 1 == 1 {
			odd_count += 1;
		} else {
			even_count += 1;
		}

		if *i as f64  > average {
			gt_average_count += 1;
		}
	}

	println!("Odd count: {}\nEven count: {}", odd_count, even_count);
	println!("Greater than average count: {}", gt_average_count);
	
}

// fn even_odd_count(x: usize, mut odd: i32, mut even: i32) -> (i32, i32) {
// 	if x & 1 == 1 {
// 		odd += 1;
// 	} else {
// 		even += 1;
// 	}
// 	(odd, even)
// }


// Exercise 3
fn exercise_3() {
    
    let student1 = Student {
        name: String::from("Alice"),
        grades: vec![
            72.0, 88.5, 91.0, 65.0, 77.5,
            83.0, 94.0, 69.5, 58.0, 86.0
        ],
    };

    let student2 = Student {
        name: String::from("Brian"),
        grades: vec![
            45.0, 67.5, 73.0, 81.5, 90.0,
            54.0, 62.5, 79.0, 88.0, 70.5
        ],
    };

    let student3 = Student {
        name: String::from("Clara"),
        grades: vec![
            92.0, 84.5, 76.0, 88.0, 95.5,
            67.0, 73.5, 81.0, 59.0, 90.0
        ],
    };

	print_report(&student1);
	print_report(&student2);
	print_report(&student3);
}

#[derive(Debug)]
struct Student {
    name: String,
    grades: Vec<f64>,
}

fn new_student(name: &str) -> Student {
	Student {
		name: name.to_string(),
		grades: Vec::new()
	}
}

fn add_grade(student: &mut Student, grade: f64) {
	student.grades.push(grade);
	println!("Grade added");
}

fn average_grade(student: &Student) -> f64 {
	let grades = &student.grades;
	if grades.len() == 0 {
		return 0.0;
	}
	let mut sum = 0.0;
	for i in grades {
		sum += i;
	}
	sum / grades.len() as f64
}

fn highest_grade(student: &Student) -> f64 {
	let grades = &student.grades;
	let mut highest = grades[0];
	for i in 1..grades.len() {
		if grades[i] > highest {
			highest = grades[i];
		}
	}
	highest
}

fn letter_grade(student: &Student) -> &str {
	let average = average_grade(student);
	let mut grade = "";
	if average > 89.0 {
		grade = "A";
	} else if average > 79.0 {
		grade = "B";
	} else if average > 69.0 {
		grade = "C";
	} else {
		grade = "F";
	}
	grade
}

fn print_report(student: &Student) {
	println!("\n{}'s report\n", student.name);
	println!("Marks: {:?}", student.grades);
	println!("Average: {}", average_grade(student));
	println!("Grade: {}", letter_grade(student));
}


fn main() {
   // exercise_1();
    //exercise_2();
    exercise_3();
}
