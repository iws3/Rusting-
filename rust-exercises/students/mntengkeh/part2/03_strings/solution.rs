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
    // Your solution here
}

fn main() {
   // exercise_1();
    exercise_2();
    //exercise_3();
}
