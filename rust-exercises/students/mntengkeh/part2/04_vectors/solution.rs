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
    // Your solution here
}

fn main() {
   // exercise_1();
    exercise_2();
    exercise_3();
}
