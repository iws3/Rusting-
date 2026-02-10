// // fn main() {
// //     let mut y=4;
// //     println!("The value of y is: {}", y);
// //     // now change the value of y
// //     y=10;
// //     println!("The value of y is now: {}", y)


// // }
// // QUESTION 1:
// // EXPLAIN DIFFERENCES BETWEEN SHADOWING AND MUTATION IN RUST
// // when should we use shadowing vs mutations



// // Each let creates a new variable that shadows the previous one. This is different from mutation because each x is actually a separate variable. You can even change the type when shadowing:

// // fn main() {
// //     let x=5;
// //     // this is shadowing and not mutation
// //     let x=x+1;
// //     let x=x*3;
// //     println!("The value of x is : {}", x)
// // }

// // We can even change the data type when shadowing

// // fn main(){
// //     let spaces="    ";
// //     let spaces=spaces.len();
// //     println!("Number of spaces: {}", spaces);
// // }


// // Try doing that with mutation and it won't work:
// // fn main() {
// //     let mut spaces="  ";
// //     spaces=spaces.len();
// // }


// // Data Types:
// // 1. Integers 
// // i for sign and u for unsigned
// // understanding i32

// // fn main(){
// //     // 8 bits -128 to 127
// //     let tiny:i8=-24;
// //     println!("The value of tiny: {}", tiny);
// //     // 16 bits: -32,768 to 32,767
// //     let small:i16=-32_768;
// //     println!("The value of small: {}", small);
// //     let normal:i32=-2_000_000;
// //     let normal_unsized:u8=244;
// //     let pointer_sized:isize=100;
// //     let u_pointer_sized:usize=100;
// //     println!("The value of 1 is: {} and 2 is: {}", pointer_sized, u_pointer_sized);

// //     println!("All these numbers live in different amounts of memory!");

// // }
// // explain swc: in nexjs , programming and what is it


// // floats
// // fn main(){
// //     // write about floats here
// //     let small_decimal:f32=3.14159;
// //     let precise_decimal:f64=3.14159265358979;
// //     // when you don't specify rust assumes f64
// //     let pi=2.5;
// //     println!("f32 is less precise: {}", small_decimal);
// //     println!("f64 gives more decimal places: {}", precise_decimal)
// // }

// // booleans
// // fn main() {
// //     let is_rust_awesome:bool=true;
// //     let is_this_javascript:bool=false;
// //     // despite only needing 1bit, a bool takes up 1 byte in the memory
// //     // this is for efficiency-modern CPUs work with byte not bits
// //     println!("Rust is awesome: {}", is_rust_awesome)
// // }

// // Character : Unicodes all the way


// // COMPOUND TYPES
// // 1. Turples: Fixed size heterogeneous Collection
// // A turple group together values of different types into one compound type with fixed length:


// // fn main() {
// //     let person:(String, i32, f64)=(String::from("Alice"), 30, 5.6);
// //     let name=&person.0;
// //     let age=person.1;
// //     let height=person.2;
// //     println!("{} is {} years old and  {} feat tall", name, age, height);
// //     let (person_name, person_age, person_height)=person;
// //     println!("Destructured: {}, {}, {}", person_name, person_age, person_height);
// //     // let unit_value:()=();
// // }




// // Control Flow: Conditionals and Loops

// // Now let's talk about controlling the flow of your program's execution.
// // If Expressions: Making Decisions
// // In Rust, if statements are actually expressions, meaning they return values:




// // fn main() {
// //     let number = 7;
    
// //     // Standard if statement
// //     if number < 5 {
// //         println!("number is less than 5");
// //     } else if number == 5 {
// //         println!("number equals 5");
// //     } else {
// //         println!("number is greater than 5");
// //     }
    
// //     // if as an expression—assigning its result
// //     let description = if number % 2 == 0 {
// //         "even"  // Note: no semicolon! This is the return value
// //     } else {
// //         "odd"
// //     };
    
// //     println!("The number is {}", description);
    
// //     // Both branches must return the same type
// //     // This won't compile:
// //     // let bad = if number > 5 { 10 } else { "ten" };
// // }




// // Note that conditions must always be booleans. Unlike JavaScript or Python, Rust won't automatically convert other values to booleans:


// // fn main() {
// //     let number = 3;
    
// //     // This won't work:
// //     // if number { println!("number is truthy"); }
    
// //     // You must be explicit:
// //     if number != 0 {
// //         println!("number is not zero");
// //     }
// // }


// // This might feel verbose, but it prevents a whole class of bugs where you accidentally use a value as a boolean.




// // 5. Loops: Repetition Three Ways

// // Rust gives you three looping constructs, each with specific use cases.
// // 5.1 The infinite loop:


// // fn main() {
// //     let mut counter=0;
// //     let result=loop {
// //         counter+=1;
// //         if counter==10 {
// //             break counter *2;
// //             // break can return a value
// //         }
// //     }
// // println!("The result is {}", result);
// // print 20
// // }

// // 5.2 The while Loop

// // fn main() {
// //     let mut number=3;
// //     while number!=0 {
// //         println!("{}!", number);
// //         number-=3;

// //     }

// //     println!("LIFTOFF!");
// // }

// // NOTE: USE A WHILE LOOP WHEN YOU HAVE A CONDITION TO CHECK, BUT DON'T KNOW IN ADVANCE HOW MANY ITERATIONS YOU'LL NEED

// // 5.2 THE FOR LOOP

// // fn main() {
// //     for number in 1..6{
// //         println("Number: {}", number);
// //     }

// //     for number in 1..=5 {
// //         println!("Number: {}", number);
// //     }

// //     let animals=["cat", "dog", "bird", "fish"];

// //     for animal in animals.iter() {
// //         println!("I like {}s", animal);
// //     }

// //     for (index, value) in animals.iter().enumerate() {
// //         println!("Animal {} is a {}", index, value);
// //     }
// // }

// //  The for loop is your go-to for iterating over collections. The .iter() method creates an iterator over the array, and .enumerate() gives you both the index and value.



// i am from the js, python world and i want to get into rust programming language on my windows 11 pc. i want you to break down this into 3 part series, first part will be intro and rust basics, second will be intermediate, and last is advance, i am new to rust, i want you to first of all start from why rust, fiedls in which rust code can excel,, why rust was created, who created it and now start with the first part for now where you go detaily onto setup, variables, data types, loops, conditionals and functions__ break this concept down and note and a computer science student so as much as you are simplifying thingd try to break down stuffs from first pronciples eg about int32 explain it detaily what it means, 

// you are going to break doen this things to me like andrej karpathy

// Jan 1
// Alright, let me take you on this journey into Rust from absolute first principles. I'm going to teach you the way I'd want to learn it myself—building up from fundamentals, understanding the why behind every decision.

// Why Rust Exists: The Problem It Was Born To Solve
// Let's start by understanding what problem Rust actually solves. You're coming from JavaScript and Python, so you've been living in what I call the "safe but slow" world. JavaScript and Python handle memory for you automatically through garbage collection. You never think about where your data lives in RAM, when it gets cleaned up, or whether two parts of your code are fighting over the same piece of memory. This is wonderful for productivity, but it comes with costs: performance overhead and unpredictable pauses when the garbage collector runs.

// On the other end of the spectrum, you have C and C++—the "fast but dangerous" world. These languages give you direct control over memory, which means you can write incredibly efficient code. But with that power comes responsibility: you can accidentally read memory you don't own (use-after-free), have multiple parts of your code modify the same data simultaneously causing race conditions, or forget to free memory causing leaks. According to Microsoft and Google, roughly 70% of serious security vulnerabilities in large codebases come from memory safety issues.

// Rust was created to break this false dichotomy. It asks: "Can we have C++ level performance AND Python level safety?" The answer, remarkably, is yes—but it requires rethinking how we write code.

// Graydon Hoare started Rust as a personal project at Mozilla Research around 2006, and Mozilla officially sponsored it starting in 2009. The first stable release came in 2015. The driving motivation was building a better browser engine (which became Servo), where both performance and security were absolutely critical.

// Where Rust Excels
// Rust shines in domains where you need:

// Systems programming: Operating systems, device drivers, embedded systems. Rust is being used in the Linux kernel now, which is a huge milestone.

// Performance-critical applications: Game engines, web browsers, databases. Discord famously rewrote parts of their infrastructure in Rust and saw massive performance improvements.

// Concurrent and parallel processing: Rust's ownership system makes it nearly impossible to write code with data races, which are traditionally some of the hardest bugs to find and fix.

// WebAssembly: Rust compiles to WASM beautifully, letting you write high-performance code that runs in the browser.

// Command-line tools: Tools like ripgrep (a faster grep) and exa (a modern ls) are written in Rust and are blazingly fast.

// The key insight is this: Rust checks at compile time—before your program ever runs—that your memory usage is safe. There's no garbage collector running in the background. There's no runtime overhead. Just safe, fast code.

// Part 1: Rust Basics - Building Your Foundation
// Setting Up Rust on Windows 11
// Rust has an official installer called rustup that manages everything for you. Here's how we'll get started:

// First, go to rustup.rs in your browser. Download and run the installer for Windows. During installation, you'll need Visual Studio C++ Build Tools. The installer will guide you through this—just follow the prompts and install the required components. This is necessary because Rust needs a linker and some Windows libraries to compile your programs.

// Once installed, open a new terminal (PowerShell or Windows Terminal) and type rustc --version to verify it worked. You should see something like "rustc 1.75.0" or whatever the current version is.

// You'll also get cargo, which is Rust's package manager and build tool combined. Think of it like npm for JavaScript, but more powerful. Check it with cargo --version.

// To create your first project, navigate to where you want your code to live and run cargo new hello_rust. This creates a new directory with a basic Rust project structure. Inside you'll find a Cargo.toml file (like package.json) and a src directory with main.rs inside it.

// Let's look at what cargo generated for us in main.rs:

// rust
// fn main() {
//     println!("Hello, world!");
// }
// To run this, go into your project directory and type cargo run. Cargo will compile your code and then execute it. You'll see "Hello, world!" printed to your console.

// Variables: Understanding Immutability From First Principles
// Now let's talk about variables, but we need to understand Rust's philosophy first. In Python or JavaScript, when you write x = 5 and later x = 10, you're mutating the variable. You're changing what x points to. This is convenient, but in large programs, especially concurrent ones, it becomes really hard to reason about what the value of x is at any given time.

// Rust takes a different approach: variables are immutable by default. Let me show you:

// rust
// fn main() {
//     let x = 5;  // x is now bound to the value 5
//     println!("The value of x is: {}", x);
    
//     x = 10;  // This will cause a compile error!
//     println!("The value of x is: {}", x);
// }
// If you try to compile this, Rust will refuse and tell you "cannot assign twice to immutable variable x". This might feel restrictive at first, but it's actually liberating. When you read code and see a variable, you know it won't change unless explicitly marked as mutable. This makes code much easier to understand and debug.

// To make a variable mutable, you explicitly opt in:

// rust
// fn main() {
//     let mut y = 5;  // The 'mut' keyword makes this mutable
//     println!("The value of y is: {}", y);
    
//     y = 10;  // Now this is fine
//     println!("The value of y is now: {}", y);
// }
// Here's something interesting about Rust: you can shadow variables by declaring them again with let:

// rust
// fn main() {
//     let x = 5;
//     let x = x + 1;  // This is shadowing, not mutation
//     let x = x * 2;
//     println!("The value of x is: {}", x);  // Prints 12
// }
// Each let creates a new variable that shadows the previous one. This is different from mutation because each x is actually a separate variable. You can even change the type when shadowing:

// rust
// fn main() {
//     let spaces = "   ";       // This is a string
//     let spaces = spaces.len(); // Now it's a number
//     println!("Number of spaces: {}", spaces);
// }
// Try doing that with mutation and it won't work:

// rust
// fn main() {
//     let mut spaces = "   ";
//     spaces = spaces.len();  // Error! Can't change types with mutation
// }
// Data Types: Understanding Memory Representation
// This is where we go deep. In Python and JavaScript, you don't think about how numbers are stored in memory. But Rust, like C and C++, requires you to be explicit about this. Let's understand why.

// Your computer's memory is just a giant array of bytes. A byte is 8 bits, and each bit is either 0 or 1. When you store a number, you need to decide: How many bytes will I use? And will I allow negative numbers?

// Integers: The Full Picture
// Rust gives you very precise control over integer types. The naming convention is: i for signed (can be negative), u for unsigned (only positive), followed by the number of bits.

// Let me break down i32, since you asked about it specifically:

// The i means it's a signed integer—it can represent both positive and negative numbers. The 32 means it uses 32 bits of memory, which is 4 bytes (since 8 bits equals 1 byte).

// With 32 bits, you can represent 2^32 different values, which is 4,294,967,296 different numbers. But since we're splitting them between positive and negative, we actually get a range from -2,147,483,648 to 2,147,483,647. That's negative 2 billion to positive 2 billion, roughly.

// Here's why this matters: if you try to store a number larger than 2,147,483,647 in an i32, you'll overflow. In debug mode, Rust panics and crashes your program. In release mode, it wraps around (like an old odometer rolling over). This is called "two's complement overflow" if you want to look deeper into the bit-level representation.

// Let me show you all the integer types:

// rust
// fn main() {
//     let tiny: i8 = -128;        // 8 bits: -128 to 127
//     let small: i16 = -32_768;   // 16 bits: -32,768 to 32,767
//     let normal: i32 = -2_000_000; // 32 bits: about -2 billion to +2 billion
//     let big: i64 = -9_000_000_000; // 64 bits: huge range
//     let huge: i128 = 170_000_000_000_000_000_000; // 128 bits: astronomical
    
//     // Unsigned versions (no negatives, so range shifts up)
//     let u_tiny: u8 = 255;       // 0 to 255
//     let u_small: u16 = 65_535;  // 0 to 65,535
//     let u_normal: u32 = 4_000_000; // 0 to about 4 billion
//     let u_big: u64 = 18_000_000_000; // 0 to huge
    
//     // Special types
//     let pointer_sized: isize = 100; // Size matches your CPU architecture
//     let u_pointer_sized: usize = 100; // Used for indexing collections
    
//     println!("All these numbers live in different amounts of memory!");
// }
// Notice I wrote numbers like 32_768 with underscores? Rust lets you do this for readability, and it ignores the underscores completely.

// The isize and usize types are special—they match your computer's architecture. On a 64-bit computer (which you have), they're 64 bits. On a 32-bit system, they're 32 bits. You'll use usize constantly for array indexing and sizes.

// Floating Point: Numbers With Decimals
// For numbers with decimal points, Rust gives you two choices:

// rust
// fn main() {
//     let small_decimal: f32 = 3.14159; // 32 bits, single precision
//     let precise_decimal: f64 = 3.14159265358979; // 64 bits, double precision (default)
    
//     let pi = 2.5; // When you don't specify, Rust assumes f64
    
//     println!("f32 is less precise: {}", small_decimal);
//     println!("f64 gives you more decimal places: {}", precise_decimal);
// }
// Floating point numbers are stored using the IEEE 754 standard, which is fascinating but complex. The key thing to know is they're approximations. They can't represent all decimal numbers exactly, which is why 0.1 + 0.2 doesn't exactly equal 0.3 in any programming language. Use f64 unless you specifically need to save memory.

// Booleans: The Simplest Type
// Booleans are straightforward but worth understanding:

// rust
// fn main() {
//     let is_rust_awesome: bool = true;
//     let is_this_javascript: bool = false;
    
//     // Despite only needing 1 bit, a bool takes 1 byte in memory
//     // This is for efficiency—modern CPUs work with bytes, not bits
//     println!("Rust is awesome: {}", is_rust_awesome);
// }
// Characters: Unicode All The Way
// Here's where Rust differs significantly from C and surprisingly from Python 3. A char in Rust is not 1 byte—it's 4 bytes, because it represents a Unicode Scalar Value:

// rust
// fn main() {
//     let letter: char = 'z';
//     let emoji: char = '😊';
//     let chinese: char = '中';
    
//     // All of these are valid chars and each takes 4 bytes
//     println!("{}, {}, {}", letter, emoji, chinese);
// }
// Note the single quotes for chars versus double quotes for strings (which we'll get to shortly). This 4-byte size means a char can represent any Unicode character directly. In many languages, you need multiple "characters" to represent an emoji, but not in Rust.

// Compound Types: Building Complexity
// Now let's talk about combining basic types into more complex structures.

// Tuples: Fixed-Size Heterogeneous Collections
// A tuple groups together values of different types into one compound type with a fixed length:

// rust
// fn main() {
//     // Tuple holding different types
//     let person: (String, i32, f64) = (String::from("Alice"), 30, 5.6);
    
//     // Accessing tuple elements with dot notation
//     let name = &person.0;    // First element (String)
//     let age = person.1;      // Second element (i32)
//     let height = person.2;   // Third element (f64)
    
//     println!("{} is {} years old and {} feet tall", name, age, height);
    
//     // Destructuring a tuple
//     let (person_name, person_age, person_height) = person;
//     println!("Destructured: {}, {}, {}", person_name, person_age, person_height);
    
//     // The unit type—an empty tuple
//     let unit_value: () = ();
//     // Functions that don't return anything actually return ()
// }
// The last example, (), is called the "unit type". It's Rust's way of representing "nothing" or "void" from C. Every function that doesn't explicitly return something actually returns the unit type.

// Arrays: Fixed-Size Homogeneous Collections
// Arrays in Rust are different from JavaScript arrays—they have a fixed size determined at compile time:

// rust
// fn main() {
//     // Array of 5 integers, all on the stack
//     let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    
//     // Accessing elements
//     let first = numbers[0];
//     let second = numbers[1];
    
//     println!("First: {}, Second: {}", first, second);
    
//     // Create an array with 10 zeros
//     let zeros = [0; 10]; // Shorthand for [0,0,0,0,0,0,0,0,0,0]
    
//     // Get the length
//     let length = numbers.len();
//     println!("Array has {} elements", length);
    
//     // This will panic at runtime if index is out of bounds
//     // let invalid = numbers[10]; // Don't do this!
// }
// The key difference from JavaScript: once you create an array with size 5, it's always size 5. You can't push or pop elements. The size is part of the type itself. If you need dynamic sizing, you'll use a Vec<T> (vector), which we'll cover in Part 2.

// Control Flow: Conditionals and Loops
// Now let's talk about controlling the flow of your program's execution.

// If Expressions: Making Decisions
// In Rust, if statements are actually expressions, meaning they return values:

// rust
// fn main() {
//     let number = 7;
    
//     // Standard if statement
//     if number < 5 {
//         println!("number is less than 5");
//     } else if number == 5 {
//         println!("number equals 5");
//     } else {
//         println!("number is greater than 5");
//     }
    
//     // if as an expression—assigning its result
//     let description = if number % 2 == 0 {
//         "even"  // Note: no semicolon! This is the return value
//     } else {
//         "odd"
//     };
    
//     println!("The number is {}", description);
    
//     // Both branches must return the same type
//     // This won't compile:
//     // let bad = if number > 5 { 10 } else { "ten" };
// }
// Note that conditions must always be booleans. Unlike JavaScript or Python, Rust won't automatically convert other values to booleans:

// rust
// fn main() {
//     let number = 3;
    
//     // This won't work:
//     // if number { println!("number is truthy"); }
    
//     // You must be explicit:
//     if number != 0 {
//         println!("number is not zero");
//     }
// }
// This might feel verbose, but it prevents a whole class of bugs where you accidentally use a value as a boolean.

// Loops: Repetition Three Ways
// Rust gives you three looping constructs, each with specific use cases.

// The infinite loop:

// rust
// fn main() {
//     let mut counter = 0;
    
//     // loop runs forever until you break out
//     let result = loop {
//         counter += 1;
        
//         if counter == 10 {
//             break counter * 2; // break can return a value!
//         }
//     };
    
//     println!("The result is {}", result); // Prints 20
// }
// Notice that break can return a value, making the loop an expression. This is useful when you're searching for something and want to return what you found.

// The while loop:

// rust
// fn main() {
//     let mut number = 3;
    
//     while number != 0 {
//         println!("{}!", number);
//         number -= 1;
//     }
    
//     println!("LIFTOFF!");
// }
// Use while when you have a condition to check but don't know in advance how many iterations you'll need.

// The for loop—the workhorse:

// rust
// fn main() {
//     // Looping through a range
//     for number in 1..6 {  // 1..6 means 1,2,3,4,5 (excludes 6)
//         println!("Number: {}", number);
//     }
    
//     // Inclusive range
//     for number in 1..=5 {  // 1..=5 means 1,2,3,4,5 (includes 5)
//         println!("Number: {}", number);
//     }
    
//     // Looping through an array
//     let animals = ["cat", "dog", "bird", "fish"];
    
//     for animal in animals.iter() {
//         println!("I like {}s", animal);
//     }
    
//     // With index and value
//     for (index, value) in animals.iter().enumerate() {
//         println!("Animal {} is a {}", index, value);
//     }
// }
// The for loop is your go-to for iterating over collections. The .iter() method creates an iterator over the array, and .enumerate() gives you both the index and value.

// Functions: Building Blocks of Abstraction


fn say_hello(){
    println!("Hello Rust!");
}

fn greet_person(name:&str, age:i32) {
    println!("{} is {} years old", name, age);
}

fn add_numbers(x:i32, y:i32)->i32 {
    // function with a return value, no semilcolon, this is an expression that gets reurned
    x+y
    // or return x+y;
    // no semicolon infront of expressions
}

// FUNCTIONS demonstrating the differences between expressions and statements

fn complex_function(x:i32)->i32 {
    let y={
        let inner=x*2;
        inner+1
        // no semicolon above, means , this is the value the block returns
    };
    // semicolon above because let is a statement

    y+5
    // this is returned from the function
}


// fn main(){
//     say_hello();
//     greet_person("jude", 23);
//     let sum=add_numbers(5, 7);
//     println!("5+7={}", sum);

//     let result=complex_function(3);
//     println!("complex_function(3) = {}", result);


// }



// Here's the crucial concept: Rust distinguishes between statements and expressions. An expression evaluates to a value. A statement performs an action but doesn't return a value. In the function above, x + y is an expression (no semicolon), so its value gets returned. If you added a semicolon (x + y;), it would become a statement and return nothing (actually the unit type ()), causing a compile error because the function signature promises to return an i32.




fn main() {
    let s1=String::from("hello");
    let s2=s1;
    // above we have transform ownership from s1 to s2
    println!("{}", s1);
    // above won't compile ! s1 no longer iwns data
    println!("{}", s2);
    // This works fine because s2 is the owner
} 
// s2 goes out of scope here, as teh string data is automatically freed


// FUNCTIONS and Ownership: Passing Values
// Ownership rules apply to functions calls too. whhen you pass a value to a function, ownership transfer to that function

fn main() {
    let s=String::from("hello");
    takes_ownership(s);
    // s ownerships moves to the function
    println!("{}", s);
    // won't compile! s no longer owns the data
    let x=5;
    makes_copy(x);
    // x is copied because i32 implements copy
    println!("{}",x);
    // above line is a very valid
}

fn takes_ownership(name_string:String) {
    println!("{}", name_string);
}


fn makes_copy(some_integer:i32){
  println!("{}", some_integer);  
}

// Notice what happens. When we call takes_ownership with s, the String data moves into the function parameter. After that call, s is no longer valid in main. When the function returns, some_string goes out of scope and the String data is automatically freed. This is ownership rule three in action.
// For the integer, because i32 implements Copy, the value is copied into the function. The original x remains valid in main.
// If you want to get ownership back from a function, you can return the value:


// IF YOU WANT TO GET OWNERSHIP BACK FROM A FUNCTION YOU CSN RETURN THE VALUE

fn takes_and_returns(a_string:String)->String {
    println!("{}", a_string);
    a_string
    // RETURN ownership of the string to the caller
}


fn main(){
    let s1=String::from("hello");
    let s2=takes_and_returns(s);
    // s1 moves in and new value moves out
    println!("{}", s2);
}


// References and Borrowing: 




// fn transfer_ownership(a:String)->String{
//     print("a string: {}", a_string)

// }



// Borrowing is Rust's way of letting you use a value without taking ownership of it. You create a reference to a value, which is like a pointer in C but with strict compile-time guarantees about safety.

fn main() {
    let s1=String::from("hello");
    // &s1 creates a reference to s1
    let len=calculate_len(&s);
    // s1 is still valid
    println("The lemmgt of '{}' is {}", s1, len);
}

// calculate the lenght of s1
// we can use the reference to call methods

fn calculate_len(s:&String)->usize {
 s.len()  
//  s goes out of scope but it doesnt own the string the dat
// so nothing is freed 
}