// ============================================
// Student: mntengkeh
// Topic: Strings (Part 2, Day 11)
// Date: 2026-03-01
// ============================================

// Exercise 1
fn exercise_1() {
    // Your solution here
}

// Exercise 2
fn exercise_2() {
    let text_o = String::from("  The quick brown fox jumps over the lazy dog.   ");
		let text = text_o.trim();

		println!("Trimmed text: {}", text);
		let text = text.to_uppercase();
		println!("Uppercase text: {}", text);
		let text = text.to_lowercase();
		println!("Lowercase text: {}", text);
		let text = text.replace("fox", "🦀");
		println!("Fox-replaced: {}\n", text);
		let mut o_count: u32 = 0;
		let mut contains_quick: bool = false;

		for (i, word) in text_o.trim().split_whitespace().enumerate() {
			for c in word.as_bytes() {
				if *c == b'o'{
					o_count += 1;
					break;
				}
			}
			println!("{}. {}", i, word);
			if word == "quick" {
				contains_quick = true;
			}

		}
		println!("\n{} words contain the letter 'o'", o_count);
		println!("\nText contains substring 'quick': {}", contains_quick);
		let words: String = text_o.trim().split_whitespace().collect::<Vec<&str>>().join(" | ");
		println!("String instance: \n{}", words);

}
// Exercise 3
fn exercise_3() {
    let multilingual = String::from("Hello Привет 你好 🌍");
		println!("String: {}", multilingual);
		// the values printed below are different because a single character in rust is represented 
		// using 4 bytes as opposed to one in other languages. 
		// characters i rust use between 1 and 4 bytes inclusive when collected in a string
		println!("Number of bytes: {}", multilingual.len());
		println!("Number of unicode characters: {}", multilingual.chars().count());

		for (i, data) in multilingual.chars().enumerate() {
			println!("{}. {}", i, data);
		}

		// won't work
		//let c = multilingual[0];

		let first = multilingual.chars().next();
		println!("First character: {}", first.unwrap());

		let mut ascii_string = String::new();
		for c in multilingual.chars() {
			if c.is_ascii() {
				ascii_string.push(c);
			}
		}

		println!("String with only ascii characters: {}", ascii_string);

}

fn main() {
   // exercise_1();
    //exercise_2();
    exercise_3();
}
