// // // // // fn main() {
// // // // //     let mut y=4;
// // // // //     println!("The value of y is: {}", y);
// // // // //     // now change the value of y
// // // // //     y=10;
// // // // //     println!("The value of y is now: {}", y)

// // // // // }
// // // // // QUESTION 1:
// // // // // EXPLAIN DIFFERENCES BETWEEN SHADOWING AND MUTATION IN RUST
// // // // // when should we use shadowing vs mutations

// // // // // Each let creates a new variable that shadows the previous one. This is different from mutation because each x is actually a separate variable. You can even change the type when shadowing:

// // // // // fn main() {
// // // // //     let x=5;
// // // // //     // this is shadowing and not mutation
// // // // //     let x=x+1;
// // // // //     let x=x*3;
// // // // //     println!("The value of x is : {}", x)
// // // // // }

// // // // // We can even change the data type when shadowing

// // // // // fn main(){
// // // // //     let spaces="    ";
// // // // //     let spaces=spaces.len();
// // // // //     println!("Number of spaces: {}", spaces);
// // // // // }

// // // // // Try doing that with mutation and it won't work:
// // // // // fn main() {
// // // // //     let mut spaces="  ";
// // // // //     spaces=spaces.len();
// // // // // }

// // // // // Data Types:
// // // // // 1. Integers
// // // // // i for sign and u for unsigned
// // // // // understanding i32

// // // // // fn main(){
// // // // //     // 8 bits -128 to 127
// // // // //     let tiny:i8=-24;
// // // // //     println!("The value of tiny: {}", tiny);
// // // // //     // 16 bits: -32,768 to 32,767
// // // // //     let small:i16=-32_768;
// // // // //     println!("The value of small: {}", small);
// // // // //     let normal:i32=-2_000_000;
// // // // //     let normal_unsized:u8=244;
// // // // //     let pointer_sized:isize=100;
// // // // //     let u_pointer_sized:usize=100;
// // // // //     println!("The value of 1 is: {} and 2 is: {}", pointer_sized, u_pointer_sized);

// // // // //     println!("All these numbers live in different amounts of memory!");

// // // // // }
// // // // // explain swc: in nexjs , programming and what is it

// // // // // floats
// // // // // fn main(){
// // // // //     // write about floats here
// // // // //     let small_decimal:f32=3.14159;
// // // // //     let precise_decimal:f64=3.14159265358979;
// // // // //     // when you don't specify rust assumes f64
// // // // //     let pi=2.5;
// // // // //     println!("f32 is less precise: {}", small_decimal);
// // // // //     println!("f64 gives more decimal places: {}", precise_decimal)
// // // // // }

// // // // // booleans
// // // // // fn main() {
// // // // //     let is_rust_awesome:bool=true;
// // // // //     let is_this_javascript:bool=false;
// // // // //     // despite only needing 1bit, a bool takes up 1 byte in the memory
// // // // //     // this is for efficiency-modern CPUs work with byte not bits
// // // // //     println!("Rust is awesome: {}", is_rust_awesome)
// // // // // }

// // // // // Character : Unicodes all the way

// // // // // COMPOUND TYPES
// // // // // 1. Turples: Fixed size heterogeneous Collection
// // // // // A turple group together values of different types into one compound type with fixed length:

// // // // // fn main() {
// // // // //     let person:(String, i32, f64)=(String::from("Alice"), 30, 5.6);
// // // // //     let name=&person.0;
// // // // //     let age=person.1;
// // // // //     let height=person.2;
// // // // //     println!("{} is {} years old and  {} feat tall", name, age, height);
// // // // //     let (person_name, person_age, person_height)=person;
// // // // //     println!("Destructured: {}, {}, {}", person_name, person_age, person_height);
// // // // //     // let unit_value:()=();
// // // // // }

// // // // // Control Flow: Conditionals and Loops

// // // // // Now let's talk about controlling the flow of your program's execution.
// // // // // If Expressions: Making Decisions
// // // // // In Rust, if statements are actually expressions, meaning they return values:

// // // // // fn main() {
// // // // //     let number = 7;

// // // // //     // Standard if statement
// // // // //     if number < 5 {
// // // // //         println!("number is less than 5");
// // // // //     } else if number == 5 {
// // // // //         println!("number equals 5");
// // // // //     } else {
// // // // //         println!("number is greater than 5");
// // // // //     }

// // // // //     // if as an expression—assigning its result
// // // // //     let description = if number % 2 == 0 {
// // // // //         "even"  // Note: no semicolon! This is the return value
// // // // //     } else {
// // // // //         "odd"
// // // // //     };

// // // // //     println!("The number is {}", description);

// // // // //     // Both branches must return the same type
// // // // //     // This won't compile:
// // // // //     // let bad = if number > 5 { 10 } else { "ten" };
// // // // // }

// // // // // Note that conditions must always be booleans. Unlike JavaScript or Python, Rust won't automatically convert other values to booleans:

// // // // // fn main() {
// // // // //     let number = 3;

// // // // //     // This won't work:
// // // // //     // if number { println!("number is truthy"); }

// // // // //     // You must be explicit:
// // // // //     if number != 0 {
// // // // //         println!("number is not zero");
// // // // //     }
// // // // // }

// // // // // This might feel verbose, but it prevents a whole class of bugs where you accidentally use a value as a boolean.

// // // // // 5. Loops: Repetition Three Ways

// // // // // Rust gives you three looping constructs, each with specific use cases.
// // // // // 5.1 The infinite loop:

// // // // // fn main() {
// // // // //     let mut counter=0;
// // // // //     let result=loop {
// // // // //         counter+=1;
// // // // //         if counter==10 {
// // // // //             break counter *2;
// // // // //             // break can return a value
// // // // //         }
// // // // //     }
// // // // // println!("The result is {}", result);
// // // // // print 20
// // // // // }

// // // // // 5.2 The while Loop

// // // // // fn main() {
// // // // //     let mut number=3;
// // // // //     while number!=0 {
// // // // //         println!("{}!", number);
// // // // //         number-=3;

// // // // //     }

// // // // //     println!("LIFTOFF!");
// // // // // }

// // // // // NOTE: USE A WHILE LOOP WHEN YOU HAVE A CONDITION TO CHECK, BUT DON'T KNOW IN ADVANCE HOW MANY ITERATIONS YOU'LL NEED

// // // // // 5.2 THE FOR LOOP

// // // // // fn main() {
// // // // //     for number in 1..6{
// // // // //         println("Number: {}", number);
// // // // //     }

// // // // //     for number in 1..=5 {
// // // // //         println!("Number: {}", number);
// // // // //     }

// // // // //     let animals=["cat", "dog", "bird", "fish"];

// // // // //     for animal in animals.iter() {
// // // // //         println!("I like {}s", animal);
// // // // //     }

// // // // //     for (index, value) in animals.iter().enumerate() {
// // // // //         println!("Animal {} is a {}", index, value);
// // // // //     }
// // // // // }

// // // // //  The for loop is your go-to for iterating over collections. The .iter() method creates an iterator over the array, and .enumerate() gives you both the index and value.

// // // // i am from the js, python world and i want to get into rust programming language on my windows 11 pc. i want you to break down this into 3 part series, first part will be intro and rust basics, second will be intermediate, and last is advance, i am new to rust, i want you to first of all start from why rust, fiedls in which rust code can excel,, why rust was created, who created it and now start with the first part for now where you go detaily onto setup, variables, data types, loops, conditionals and functions__ break this concept down and note and a computer science student so as much as you are simplifying thingd try to break down stuffs from first pronciples eg about int32 explain it detaily what it means,

// // // // you are going to break doen this things to me like andrej karpathy

// // // // Jan 1
// // // // Alright, let me take you on this journey into Rust from absolute first principles. I'm going to teach you the way I'd want to learn it myself—building up from fundamentals, understanding the why behind every decision.

// // // // Why Rust Exists: The Problem It Was Born To Solve
// // // // Let's start by understanding what problem Rust actually solves. You're coming from JavaScript and Python, so you've been living in what I call the "safe but slow" world. JavaScript and Python handle memory for you automatically through garbage collection. You never think about where your data lives in RAM, when it gets cleaned up, or whether two parts of your code are fighting over the same piece of memory. This is wonderful for productivity, but it comes with costs: performance overhead and unpredictable pauses when the garbage collector runs.

// // // // On the other end of the spectrum, you have C and C++—the "fast but dangerous" world. These languages give you direct control over memory, which means you can write incredibly efficient code. But with that power comes responsibility: you can accidentally read memory you don't own (use-after-free), have multiple parts of your code modify the same data simultaneously causing race conditions, or forget to free memory causing leaks. According to Microsoft and Google, roughly 70% of serious security vulnerabilities in large codebases come from memory safety issues.

// // // // Rust was created to break this false dichotomy. It asks: "Can we have C++ level performance AND Python level safety?" The answer, remarkably, is yes—but it requires rethinking how we write code.

// // // // Graydon Hoare started Rust as a personal project at Mozilla Research around 2006, and Mozilla officially sponsored it starting in 2009. The first stable release came in 2015. The driving motivation was building a better browser engine (which became Servo), where both performance and security were absolutely critical.

// // // // Where Rust Excels
// // // // Rust shines in domains where you need:

// // // // Systems programming: Operating systems, device drivers, embedded systems. Rust is being used in the Linux kernel now, which is a huge milestone.

// // // // Performance-critical applications: Game engines, web browsers, databases. Discord famously rewrote parts of their infrastructure in Rust and saw massive performance improvements.

// // // // Concurrent and parallel processing: Rust's ownership system makes it nearly impossible to write code with data races, which are traditionally some of the hardest bugs to find and fix.

// // // // WebAssembly: Rust compiles to WASM beautifully, letting you write high-performance code that runs in the browser.

// // // // Command-line tools: Tools like ripgrep (a faster grep) and exa (a modern ls) are written in Rust and are blazingly fast.

// // // // The key insight is this: Rust checks at compile time—before your program ever runs—that your memory usage is safe. There's no garbage collector running in the background. There's no runtime overhead. Just safe, fast code.

// // // // Part 1: Rust Basics - Building Your Foundation
// // // // Setting Up Rust on Windows 11
// // // // Rust has an official installer called rustup that manages everything for you. Here's how we'll get started:

// // // // First, go to rustup.rs in your browser. Download and run the installer for Windows. During installation, you'll need Visual Studio C++ Build Tools. The installer will guide you through this—just follow the prompts and install the required components. This is necessary because Rust needs a linker and some Windows libraries to compile your programs.

// // // // Once installed, open a new terminal (PowerShell or Windows Terminal) and type rustc --version to verify it worked. You should see something like "rustc 1.75.0" or whatever the current version is.

// // // // You'll also get cargo, which is Rust's package manager and build tool combined. Think of it like npm for JavaScript, but more powerful. Check it with cargo --version.

// // // // To create your first project, navigate to where you want your code to live and run cargo new hello_rust. This creates a new directory with a basic Rust project structure. Inside you'll find a Cargo.toml file (like package.json) and a src directory with main.rs inside it.

// // // // Let's look at what cargo generated for us in main.rs:

// // // // rust
// // // // fn main() {
// // // //     println!("Hello, world!");
// // // // }
// // // // To run this, go into your project directory and type cargo run. Cargo will compile your code and then execute it. You'll see "Hello, world!" printed to your console.

// // // // Variables: Understanding Immutability From First Principles
// // // // Now let's talk about variables, but we need to understand Rust's philosophy first. In Python or JavaScript, when you write x = 5 and later x = 10, you're mutating the variable. You're changing what x points to. This is convenient, but in large programs, especially concurrent ones, it becomes really hard to reason about what the value of x is at any given time.

// // // // Rust takes a different approach: variables are immutable by default. Let me show you:

// // // // rust
// // // // fn main() {
// // // //     let x = 5;  // x is now bound to the value 5
// // // //     println!("The value of x is: {}", x);

// // // //     x = 10;  // This will cause a compile error!
// // // //     println!("The value of x is: {}", x);
// // // // }
// // // // If you try to compile this, Rust will refuse and tell you "cannot assign twice to immutable variable x". This might feel restrictive at first, but it's actually liberating. When you read code and see a variable, you know it won't change unless explicitly marked as mutable. This makes code much easier to understand and debug.

// // // // To make a variable mutable, you explicitly opt in:

// // // // rust
// // // // fn main() {
// // // //     let mut y = 5;  // The 'mut' keyword makes this mutable
// // // //     println!("The value of y is: {}", y);

// // // //     y = 10;  // Now this is fine
// // // //     println!("The value of y is now: {}", y);
// // // // }
// // // // Here's something interesting about Rust: you can shadow variables by declaring them again with let:

// // // // rust
// // // // fn main() {
// // // //     let x = 5;
// // // //     let x = x + 1;  // This is shadowing, not mutation
// // // //     let x = x * 2;
// // // //     println!("The value of x is: {}", x);  // Prints 12
// // // // }
// // // // Each let creates a new variable that shadows the previous one. This is different from mutation because each x is actually a separate variable. You can even change the type when shadowing:

// // // // rust
// // // // fn main() {
// // // //     let spaces = "   ";       // This is a string
// // // //     let spaces = spaces.len(); // Now it's a number
// // // //     println!("Number of spaces: {}", spaces);
// // // // }
// // // // Try doing that with mutation and it won't work:

// // // // rust
// // // // fn main() {
// // // //     let mut spaces = "   ";
// // // //     spaces = spaces.len();  // Error! Can't change types with mutation
// // // // }
// // // // Data Types: Understanding Memory Representation
// // // // This is where we go deep. In Python and JavaScript, you don't think about how numbers are stored in memory. But Rust, like C and C++, requires you to be explicit about this. Let's understand why.

// // // // Your computer's memory is just a giant array of bytes. A byte is 8 bits, and each bit is either 0 or 1. When you store a number, you need to decide: How many bytes will I use? And will I allow negative numbers?

// // // // Integers: The Full Picture
// // // // Rust gives you very precise control over integer types. The naming convention is: i for signed (can be negative), u for unsigned (only positive), followed by the number of bits.

// // // // Let me break down i32, since you asked about it specifically:

// // // // The i means it's a signed integer—it can represent both positive and negative numbers. The 32 means it uses 32 bits of memory, which is 4 bytes (since 8 bits equals 1 byte).

// // // // With 32 bits, you can represent 2^32 different values, which is 4,294,967,296 different numbers. But since we're splitting them between positive and negative, we actually get a range from -2,147,483,648 to 2,147,483,647. That's negative 2 billion to positive 2 billion, roughly.

// // // // Here's why this matters: if you try to store a number larger than 2,147,483,647 in an i32, you'll overflow. In debug mode, Rust panics and crashes your program. In release mode, it wraps around (like an old odometer rolling over). This is called "two's complement overflow" if you want to look deeper into the bit-level representation.

// // // // Let me show you all the integer types:

// // // // rust
// // // // fn main() {
// // // //     let tiny: i8 = -128;        // 8 bits: -128 to 127
// // // //     let small: i16 = -32_768;   // 16 bits: -32,768 to 32,767
// // // //     let normal: i32 = -2_000_000; // 32 bits: about -2 billion to +2 billion
// // // //     let big: i64 = -9_000_000_000; // 64 bits: huge range
// // // //     let huge: i128 = 170_000_000_000_000_000_000; // 128 bits: astronomical

// // // //     // Unsigned versions (no negatives, so range shifts up)
// // // //     let u_tiny: u8 = 255;       // 0 to 255
// // // //     let u_small: u16 = 65_535;  // 0 to 65,535
// // // //     let u_normal: u32 = 4_000_000; // 0 to about 4 billion
// // // //     let u_big: u64 = 18_000_000_000; // 0 to huge

// // // //     // Special types
// // // //     let pointer_sized: isize = 100; // Size matches your CPU architecture
// // // //     let u_pointer_sized: usize = 100; // Used for indexing collections

// // // //     println!("All these numbers live in different amounts of memory!");
// // // // }
// // // // Notice I wrote numbers like 32_768 with underscores? Rust lets you do this for readability, and it ignores the underscores completely.

// // // // The isize and usize types are special—they match your computer's architecture. On a 64-bit computer (which you have), they're 64 bits. On a 32-bit system, they're 32 bits. You'll use usize constantly for array indexing and sizes.

// // // // Floating Point: Numbers With Decimals
// // // // For numbers with decimal points, Rust gives you two choices:

// // // // rust
// // // // fn main() {
// // // //     let small_decimal: f32 = 3.14159; // 32 bits, single precision
// // // //     let precise_decimal: f64 = 3.14159265358979; // 64 bits, double precision (default)

// // // //     let pi = 2.5; // When you don't specify, Rust assumes f64

// // // //     println!("f32 is less precise: {}", small_decimal);
// // // //     println!("f64 gives you more decimal places: {}", precise_decimal);
// // // // }
// // // // Floating point numbers are stored using the IEEE 754 standard, which is fascinating but complex. The key thing to know is they're approximations. They can't represent all decimal numbers exactly, which is why 0.1 + 0.2 doesn't exactly equal 0.3 in any programming language. Use f64 unless you specifically need to save memory.

// // // // Booleans: The Simplest Type
// // // // Booleans are straightforward but worth understanding:

// // // // rust
// // // // fn main() {
// // // //     let is_rust_awesome: bool = true;
// // // //     let is_this_javascript: bool = false;

// // // //     // Despite only needing 1 bit, a bool takes 1 byte in memory
// // // //     // This is for efficiency—modern CPUs work with bytes, not bits
// // // //     println!("Rust is awesome: {}", is_rust_awesome);
// // // // }
// // // // Characters: Unicode All The Way
// // // // Here's where Rust differs significantly from C and surprisingly from Python 3. A char in Rust is not 1 byte—it's 4 bytes, because it represents a Unicode Scalar Value:

// // // // rust
// // // // fn main() {
// // // //     let letter: char = 'z';
// // // //     let emoji: char = '😊';
// // // //     let chinese: char = '中';

// // // //     // All of these are valid chars and each takes 4 bytes
// // // //     println!("{}, {}, {}", letter, emoji, chinese);
// // // // }
// // // // Note the single quotes for chars versus double quotes for strings (which we'll get to shortly). This 4-byte size means a char can represent any Unicode character directly. In many languages, you need multiple "characters" to represent an emoji, but not in Rust.

// // // // Compound Types: Building Complexity
// // // // Now let's talk about combining basic types into more complex structures.

// // // // Tuples: Fixed-Size Heterogeneous Collections
// // // // A tuple groups together values of different types into one compound type with a fixed length:

// // // // rust
// // // // fn main() {
// // // //     // Tuple holding different types
// // // //     let person: (String, i32, f64) = (String::from("Alice"), 30, 5.6);

// // // //     // Accessing tuple elements with dot notation
// // // //     let name = &person.0;    // First element (String)
// // // //     let age = person.1;      // Second element (i32)
// // // //     let height = person.2;   // Third element (f64)

// // // //     println!("{} is {} years old and {} feet tall", name, age, height);

// // // //     // Destructuring a tuple
// // // //     let (person_name, person_age, person_height) = person;
// // // //     println!("Destructured: {}, {}, {}", person_name, person_age, person_height);

// // // //     // The unit type—an empty tuple
// // // //     let unit_value: () = ();
// // // //     // Functions that don't return anything actually return ()
// // // // }
// // // // The last example, (), is called the "unit type". It's Rust's way of representing "nothing" or "void" from C. Every function that doesn't explicitly return something actually returns the unit type.

// // // // Arrays: Fixed-Size Homogeneous Collections
// // // // Arrays in Rust are different from JavaScript arrays—they have a fixed size determined at compile time:

// // // // rust
// // // // fn main() {
// // // //     // Array of 5 integers, all on the stack
// // // //     let numbers: [i32; 5] = [1, 2, 3, 4, 5];

// // // //     // Accessing elements
// // // //     let first = numbers[0];
// // // //     let second = numbers[1];

// // // //     println!("First: {}, Second: {}", first, second);

// // // //     // Create an array with 10 zeros
// // // //     let zeros = [0; 10]; // Shorthand for [0,0,0,0,0,0,0,0,0,0]

// // // //     // Get the length
// // // //     let length = numbers.len();
// // // //     println!("Array has {} elements", length);

// // // //     // This will panic at runtime if index is out of bounds
// // // //     // let invalid = numbers[10]; // Don't do this!
// // // // }
// // // // The key difference from JavaScript: once you create an array with size 5, it's always size 5. You can't push or pop elements. The size is part of the type itself. If you need dynamic sizing, you'll use a Vec<T> (vector), which we'll cover in Part 2.

// // // // Control Flow: Conditionals and Loops
// // // // Now let's talk about controlling the flow of your program's execution.

// // // // If Expressions: Making Decisions
// // // // In Rust, if statements are actually expressions, meaning they return values:

// // // // rust
// // // // fn main() {
// // // //     let number = 7;

// // // //     // Standard if statement
// // // //     if number < 5 {
// // // //         println!("number is less than 5");
// // // //     } else if number == 5 {
// // // //         println!("number equals 5");
// // // //     } else {
// // // //         println!("number is greater than 5");
// // // //     }

// // // //     // if as an expression—assigning its result
// // // //     let description = if number % 2 == 0 {
// // // //         "even"  // Note: no semicolon! This is the return value
// // // //     } else {
// // // //         "odd"
// // // //     };

// // // //     println!("The number is {}", description);

// // // //     // Both branches must return the same type
// // // //     // This won't compile:
// // // //     // let bad = if number > 5 { 10 } else { "ten" };
// // // // }
// // // // Note that conditions must always be booleans. Unlike JavaScript or Python, Rust won't automatically convert other values to booleans:

// // // // rust
// // // // fn main() {
// // // //     let number = 3;

// // // //     // This won't work:
// // // //     // if number { println!("number is truthy"); }

// // // //     // You must be explicit:
// // // //     if number != 0 {
// // // //         println!("number is not zero");
// // // //     }
// // // // }
// // // // This might feel verbose, but it prevents a whole class of bugs where you accidentally use a value as a boolean.

// // // // Loops: Repetition Three Ways
// // // // Rust gives you three looping constructs, each with specific use cases.

// // // // The infinite loop:

// // // // rust
// // // // fn main() {
// // // //     let mut counter = 0;

// // // //     // loop runs forever until you break out
// // // //     let result = loop {
// // // //         counter += 1;

// // // //         if counter == 10 {
// // // //             break counter * 2; // break can return a value!
// // // //         }
// // // //     };

// // // //     println!("The result is {}", result); // Prints 20
// // // // }
// // // // Notice that break can return a value, making the loop an expression. This is useful when you're searching for something and want to return what you found.

// // // // The while loop:

// // // // rust
// // // // fn main() {
// // // //     let mut number = 3;

// // // //     while number != 0 {
// // // //         println!("{}!", number);
// // // //         number -= 1;
// // // //     }

// // // //     println!("LIFTOFF!");
// // // // }
// // // // Use while when you have a condition to check but don't know in advance how many iterations you'll need.

// // // // The for loop—the workhorse:

// // // // rust
// // // // fn main() {
// // // //     // Looping through a range
// // // //     for number in 1..6 {  // 1..6 means 1,2,3,4,5 (excludes 6)
// // // //         println!("Number: {}", number);
// // // //     }

// // // //     // Inclusive range
// // // //     for number in 1..=5 {  // 1..=5 means 1,2,3,4,5 (includes 5)
// // // //         println!("Number: {}", number);
// // // //     }

// // // //     // Looping through an array
// // // //     let animals = ["cat", "dog", "bird", "fish"];

// // // //     for animal in animals.iter() {
// // // //         println!("I like {}s", animal);
// // // //     }

// // // //     // With index and value
// // // //     for (index, value) in animals.iter().enumerate() {
// // // //         println!("Animal {} is a {}", index, value);
// // // //     }
// // // // }
// // // // The for loop is your go-to for iterating over collections. The .iter() method creates an iterator over the array, and .enumerate() gives you both the index and value.

// // // // Functions: Building Blocks of Abstraction

// // // fn say_hello(){
// // //     println!("Hello Rust!");
// // // }

// // // fn greet_person(name:&str, age:i32) {
// // //     println!("{} is {} years old", name, age);
// // // }

// // // fn add_numbers(x:i32, y:i32)->i32 {
// // //     // function with a return value, no semilcolon, this is an expression that gets reurned
// // //     x+y
// // //     // or return x+y;
// // //     // no semicolon infront of expressions
// // // }

// // // // FUNCTIONS demonstrating the differences between expressions and statements

// // // fn complex_function(x:i32)->i32 {
// // //     let y={
// // //         let inner=x*2;
// // //         inner+1
// // //         // no semicolon above, means , this is the value the block returns
// // //     };
// // //     // semicolon above because let is a statement

// // //     y+5
// // //     // this is returned from the function
// // // }

// // // // fn main(){
// // // //     say_hello();
// // // //     greet_person("jude", 23);
// // // //     let sum=add_numbers(5, 7);
// // // //     println!("5+7={}", sum);

// // // //     let result=complex_function(3);
// // // //     println!("complex_function(3) = {}", result);

// // // // }

// // // // Here's the crucial concept: Rust distinguishes between statements and expressions. An expression evaluates to a value. A statement performs an action but doesn't return a value. In the function above, x + y is an expression (no semicolon), so its value gets returned. If you added a semicolon (x + y;), it would become a statement and return nothing (actually the unit type ()), causing a compile error because the function signature promises to return an i32.

// // // fn main() {
// // //     let s1=String::from("hello");
// // //     let s2=s1;
// // //     // above we have transform ownership from s1 to s2
// // //     println!("{}", s1);
// // //     // above won't compile ! s1 no longer iwns data
// // //     println!("{}", s2);
// // //     // This works fine because s2 is the owner
// // // }
// // // // s2 goes out of scope here, as teh string data is automatically freed

// // // // FUNCTIONS and Ownership: Passing Values
// // // // Ownership rules apply to functions calls too. whhen you pass a value to a function, ownership transfer to that function

// // // fn main() {
// // //     let s=String::from("hello");
// // //     takes_ownership(s);
// // //     // s ownerships moves to the function
// // //     println!("{}", s);
// // //     // won't compile! s no longer owns the data
// // //     let x=5;
// // //     makes_copy(x);
// // //     // x is copied because i32 implements copy
// // //     println!("{}",x);
// // //     // above line is a very valid
// // // }

// // // fn takes_ownership(name_string:String) {
// // //     println!("{}", name_string);
// // // }

// // // fn makes_copy(some_integer:i32){
// // //   println!("{}", some_integer);
// // // }

// // // // Notice what happens. When we call takes_ownership with s, the String data moves into the function parameter. After that call, s is no longer valid in main. When the function returns, some_string goes out of scope and the String data is automatically freed. This is ownership rule three in action.
// // // // For the integer, because i32 implements Copy, the value is copied into the function. The original x remains valid in main.
// // // // If you want to get ownership back from a function, you can return the value:

// // // // IF YOU WANT TO GET OWNERSHIP BACK FROM A FUNCTION YOU CSN RETURN THE VALUE

// // // fn takes_and_returns(a_string:String)->String {
// // //     println!("{}", a_string);
// // //     a_string
// // //     // RETURN ownership of the string to the caller
// // // }

// // // fn main(){
// // //     let s1=String::from("hello");
// // //     let s2=takes_and_returns(s);
// // //     // s1 moves in and new value moves out
// // //     println!("{}", s2);
// // // }

// // // // References and Borrowing:

// // // // fn transfer_ownership(a:String)->String{
// // // //     print("a string: {}", a_string)

// // // // }

// // // // Borrowing is Rust's way of letting you use a value without taking ownership of it. You create a reference to a value, which is like a pointer in C but with strict compile-time guarantees about safety.

// // // fn main() {
// // //     let s1=String::from("hello");
// // //     let len=calculate_length(&s1); //s1 creates a reference to s1

// // //     print("The length of '{}'", sq1, len); //s1 is still valid

// // // }

// // // fn calculate_length(s:&string)->usize {
// // //     s.len()
// // //     // we can use the references to call methods
// // // } //s goes out of scope but it doesn't own the the string data, so nothing is cleared [freed]

// // // // The ampersand creates a reference. When we write &s1, we're creating a reference to s1's data without taking ownership. The function parameter s: &String means "s is a reference to a String." Inside the function, we can use the reference to access the data, but when the function ends, the reference goes away without affecting the original data.
// // // // This is called borrowing because you're borrowing the value—you get to use it temporarily, but you don't own it and you have to give it back. The actual owner, s1, remains the owner throughout.

// // // // Mutable References: Borrowing with Permission to Modify

// // // // By default, references are immutable. You can read the data through them but not modify it. If you want to modify data through a reference, you need a mutable reference:

// // // fn main(){

// // //     let mut a=String::from("hello"); //Note a must be mut
// // //     change(&mut s); //Create a mutable reference with &mut
// // //     println!("{}", s); //print hello world
// // // }

// // // fn change(some_string:&mut string) {
// // //     name_string.push_str(", world");
// // // }

// // fn exercise_1() {
// //     // let mut x = 10;
// //     // x = 20;
// //     // println!("x is {}", x);

// //     // let mut greeting = "hello";
// //     // greeting = "goodbye";
// //     // println!("{}", greeting);

// //     // let max_score = 100;
// //     // println!("Max score is {}", max_score);
// //     // variables rewind
// //     let mut x: i32 = 5;
// //     let y = 34;
// //     println!("The value of y before updating it: {}", y);
// //     y = 45;
// //     println!("The value of y after updating it: {}", y);
// //     println!("The value of x is: {}", x);
// //     x = 6;
// //     println!("The value of x is: {}", x);
// //     // we can set x to a return value of a function but not const
// //     // differmetiate between const vs let
// //     // we cannot use mut in const.. when we are sure variable is not going to change

// //     // shadowing hel us to preserve mutabilityty
// //     let z:i32=6;
// //     let z:i32=z+1;
// //     const SUB_COUNT:u32=100000;
// //     // println!(SUB_COUNT);

// //     // RUST HAS 4 SCALAR DATATYES:
// //     // INteger, floating point, booleanb, character
// //     // 8bits/16bits/64bits/128/arc

// //     // COMPOUND TYPES
// //     let tup: (&str, i32)=("Let's Get Rusty!",100);
// //     // let (channel:&str, sub:i32)=tup
// //     let sub_count:i32=tup.1;

// // }

// // fn main() {
// //     exercise_1()
// // }

// // Loops in rust
// // fn main(){
// //     let a: [i32; _]=[10, 20, 30, 50];
// //     // println!("The list is: {}", a);
// //     for element in a.iter() {
// //         println!("The value is: {}", element);
// //     }

// //     // seconnd for loop
// //     for number in 1..4 {
// //         println!("{}!", number)
// //     }
// // }

// // fn main() {
// //     // erarys are static: ccreaqte an array with 8 values all set to 0's
// //     let byte: [i32; _]=[1, 3,4,5];
// //     let byte2:[i32; _]=[0; 8];
// //     // why will you not print the array?

// // }

// // fn main() {
// //     // Controlm flows
// //     // in rust. conditon must be a boolean, eg if number {} --> is going to be false
// //     let number:i32=5;
// //     if number < 10 {
// //         println!("First condition was true");
// //     }
// //     else if number < 22 {
// //         println!("Second condition was true");
// //     }

// //     else {
// //         println!("Condition was false");
// //     }
// // }

// // execute until we call break

// // fn main(){
// //     let mut counter:i32=0;
// //     let results=loop {
// //         counter+=1;
// //         println!("Running forever....");
// //         if counter==10 {
// //         break counter;

// //         }
// //         // break
// //         // adding break counter : will make this loop return counter
// //         // we can return values from this type of blocks, and if we dont add break, it runs forever
// //     };
// //     println!("The result is: {}", results);
// // }

// // ownership=> memory safety guarantee without using garabge collection

// // fn main(){
// //     let s1=String::from("hello");
// //     let s2=s1;
// //     println!("{}", s1);
// // }

// // understanding stacks/heaps
// // fn main(){
// //     fn a(){
// //         // &str-> string literal is stored in the binary and we store the reference in the stack
// //         let x:&str="hello";
// //         let y:i32=32;
// //         b()
// //     }

// //     fn b(){
// //         // stored in the heap and then store the pointer in the stack. pushing to stack is faster thatn allocated to the heap, necuase the heap spend time looking where to allocated
// //         let s=String::from("world")
// //     }
// // }

// // ownership rows in Rust

// // fn main(){
// //     // 1. Each value in rust has a variable that is called its owner
// //     // 2. There can only be one owner at a time
// //     // 3.when the owner goe out of scope the owner will be dropped and the value freed:
// //     // eg.
// //     { // s is not valid here
// //     let s:&str="hello";
// // // do something  with s,
// // // string literals are stored in binmary and are fixed and cannot be mutated
// //     } // scope is now over, and s is no longer valid

// //     // string type is stored in the heap and can muted
// //     // let s=String::from("world")

// // }

// // fn main(){
// //     let x:i32=5;
// //     let y:i32=x; //copy ... possible because they are fixed and stored in the stack and uses less memory

// //     // but dealing with string types is not possible nbecause they are stored in the heap

// //     let s1=String::from("hello");
// //     let s2=s1; // this is a move and when this happens the value of s1 is completely moved to s2 and s1 becomes invalid.

// //     println!("{}, world!", s1); // will give us an error

// //     // we can copy a string typew by cloning it
// //     // let s2=s1.clone();
// // }

// // fn main() {
// //     let s=String::from("hello");
// //     takeownership(s);
// //     // if we try to print s, we have an error becuase some strng has taken ownership
// //     println!("{}", s)
// //     // but it works for integers/float and other vars stored in the stack

// //     let x:i32=34;
// //     create_copy(x);
// //     // print x is still possible because it was copied

// // }

// // fn takeownership(some_string:String){
// //     println!("{}", some_string)

// // }

// // fn create_copy(val:i32){
// //     println!("{}", val);
// // }

// // fn main() {
// //     // what if we want to print values after passing it into functions, we use  functions with return keyword
// //     let return_val = return_string();
// //     println!("{}", return_val);
// // }

// // fn return_string() -> String {
// //     let some_string = String::from("hello");
// //     // when we return, ownership is moved to return_val above when we call the function above
// //     some_string
// // }

// // // WE CAN TAKE OWNERSHIP AND GIVE IT BACK
// // fn main(){
// //     // let s1=String::from()
// //     let s1=give_ownership();
// //     let s2=String::from("hello");
// //     let s3=takes_and_give_back(s2);
// //     println!("s1={}, s3={}", s1, s3);
// // }

// // fn give_ownership()->String {
// //     let some_string=String::from("hello");
// //     return some_string;
// // }

// // fn takes_and_give_back(a_string:String)->String {
// //     a_string
// // }

// // giving and taking back variables from functions can be tedious what if we can pass a variable without taking ownership?: that is where referncing comes in

// // REFERENCES

// // fn main() {
// //     // pass in our string here
// //     let s1 = String::from("hello");
// //     let len = calculate_length(&s1);
// //     // if we try to print s1 we have an error because its ownership hs been taken by the function. so the soltution is to borrow it to our function uisng the idea of referencing by adding & to our string type in function param to have-> &String and to when we are calling our function to have &s
// //     // references are immutrable by default
// //     // check rust book for the diagram
// //     // passing as refernces to function parameters is called borrowing

// //     println!("The length of '{}' is {}.",s1, len);
// // }

// // fn calculate_length(s: &String) -> usize {
// //     let length = s.len();
// //     length
// // }

// // handlng immutable varibales

// // mutating strings without taking ownership of the underlying value

// // fn main() {
// //     let mut s1 = String::from("hello");
// //     change(&mut s1)
// // }

// // // pass mutable reference instead tom utate value
// // fn change(some_string:&mut String) {
// //     // error because we are trying to mutate  a reference which is not mutable by default
// //     some_string.push_str(", world")
// // }

// //LIMITATION RULE_1: YOU CAN ONLY HAVE 1 MUTABLE REFERNCE TO A PARTICULAR PIECE OF DATA IN A PARTICULAR SCOPE

// // // eg
// // fn main() {
// //     let mut  s=String::from("hello");
// //     let r1=&mut s;
// //     // second mutable error problem here
// //     let r2=&mut s;
// //     // prevent race conditons
// //     println!("Printing r1={} and r2={}", r1, r2);

// // }

// // slices

// // slices do not takle ownership

// //  slices let you reference a  contigouse block of elements within a collectiom instead of referencing the entire collection without taking ownership

// // fn main() {
// //     let mut sample_string = String::from("Hello, World");

// //     let  s_sample = first_word(&sample_string);
// //     // get string slices
// //     let hello=&sample_string[..5];
// //     let whole_word=&sample_string[..];
// //     let world=&sample_string[6..];
// //     println!("The results gotten is: {}", s_sample);

// //     let word=first_word(&sample_string);
// //     sample_string.clear();

// //     println!("{}", word);
// //     // println!("The word itself is: {}", sample_string[s_sample]);

// //     // if we do s.clear, our code still works even tho string has been cleared.. reason beign our index return is loose away from string
// // }

// // write a function to take a reerence of that string and return the index of the last element [return first word]

// // ONE WAY TO DO IT:
// // after changing the format to get slices above, we change the return type from the index to return string
// // fn first_word(s: &String) -> &str {
// //     // convert string to an bytes? why that?
// //     let bytes = s.as_bytes();

// //     for (i, &item) in bytes.iter().enumerate() {
// //         if item == b' ' {
// //             // if item is space... we have = hit the end of the word
// //             // return i;
// //             // return the string slice from the begining of the word to the place where the slice was found
// //             return &s[0..i];
// //         }
// //     }
// //     // instead of return ing s.len()
// //     // return the string slice to the entire string
// //     &s[..]
// // }

// // str : string literals are  String slices:

// // ENUMS/STRUCTS ARE THE BUILDING BLOCKS FOR CREATING NEW TYPES IN RUSTS
// // grouping related data using strucs.. defining data and assocaited method using strucs and how it compares to turples
// // There are object attribute in an object orientatied programming langugae

// // create a struct for a user:
// // struct User {
// //     username:String,
// //     email:String,
// //     sign_in_coount:u64,
// //     active:bool,
// // }

// // fn main(){
// // // create a new instance:
// // let mut user1=User {
// //     email:String::from("fon@gmail.com"),
// //     username:String::from("jufe@23"),
// //     sign_in_coount:23,
// //     active:true,
// // };
// // // now lets use our function t create another user
// // let user2=build_user(String::from("Paul@email.com"),String::from("Paul"));

// // // we can get specifi values from our struct using the dot notation
// // let name=user1.username;
// // // we can also update items fdrom our struct using the dot notation first we make our suer 1 instance mut
// // user1.username=String::from("PatJet");

// // // we can create new instances using existing instances
// // let mut user3=User {
// //     email:String::from("fon@gmail.com"),
// //     username:String::from("jila"),
// //     // then copy the other ones
// //     ..user2
// // };

// // user3.username=String::from("Paulloo");
// // println!("Username for user 1 is {}:", user1.username);
// // println!("Username for user 1 is {}:", user3.username);
// // }

// // // we can use functions to construct new instances of user
// // fn build_user(email:String, username:String)->User {
// //     // Fill init short hansd syntax, instead
// //     // email:email, wee can do "email," since value and key are same

// //     User {
// //         email:email,
// //         username:username,
// //         active:true,
// //         sign_in_coount:1,
// //     }

// // }

// // turple struct [structs without name field]
// // when we want our turple to have a name and different types from other turples
// // fn main() {
// //     struct Color(i32, i32, i32);
// //     struct Point(i32, i32, i32);
// // }

// // making calculating area of a rectangle with rust structs the best thing ever

// // rewrite the program below using structs

// // fn main() {
// //     let width1=30;
// //     let height1=50;
// //     println!("The area of a rectangle is {} square pixels.", area(width1, height1))
// // }

// // fn area(width:u32, height:u32)->u32{
// // width * height
// // }

// // improve the readabilty by grouping this two variables together using tuples

// // fn main () {
// //     let rect=(30, 50);
// //     println!("The value of our area calculated is : {}: ", area(rect));
// // }

// // fn area(dimensions: (u32, u32))->u32 {
// //     dimensions.0 * dimensions.1
// // }

// // third case using structs -> its not clear what our fields in our turples are named in function signature above
// // // #[derive(Debug)]
// // #[derive(Debug)]
// // // {:#?}->make it a little pretier
// // struct Rectangle {
// //     width:u32,
// //     height:u32,
// // }
// // fn main() {
// // let rectangle=Rectangle {
// //     width:40,
// //     height:40,

// // };
// // println!("The rectangle looks like this: {:#?}", rectangle);
// // println!("The area of the rectangle is: {}", area(&rectangle));
// // }

// // fn area(rectangle: &Rectangle)->u32{
// //     rectangle.width * rectangle.height
// // }

// // to see what our rectangle instance look like we use derive traits

// // FUNCTIONS IN A STRUCT ARE CALLED  -> methods

// // use the impl method_name

// // #[derive(Debug)]

// // struct Rectangle {
// //     width: u32,
// //     height: u32,
// // }

// // // implement method to calculate area, ir should be outside of the struct
// // impl Rectangle {
// //     fn area(&self) -> u32 {
// //         // self is pointing to the struct in which we call our method on, so this time around we are not taking ownership of
// //         // it but passing a reference
// //         self.width * self.height
// //     }

// //     // methods to check if one rectangle can go inside of another:
// //     fn can_hold(&self, other: &Rectangle) -> bool {
// //         self.width * self.height > other.width * other.height
// //     }

// //     // implementing associated methods, we dont need to pass the &self property inside [they are not tide to the instance of our struct]
// //     // we can do it inside of this implementation blog but structs allows to create somany implementation blocks
// // }

// // impl Rectangle {
// //     fn square(size: u32) -> Rectangle {
// //         // Returen the rectnagle instance and pass in size for both height and widht
// //         Rectangle {
// //             width: size,
// //             height: size,
// //         }
// //     }
// //     // to call our associated method we dont use the dot notations we use the :: syntax: Rectangle::square(23);
// // }

// // fn main() {
// //     let rect = Rectangle {
// //         width: 45,
// //         height: 46,
// //     };

// //     let rect2 = Rectangle {
// //         width: 56,
// //         height: 56,
// //     };

// //     let square=Rectangle::square(34);

// //     // in lan like c++ there is a diference in way you are calling the function on a methods directly  or calling a methoid on a pointer to a object..
// //     // Rust its the same because it has something called:
// //     // explore automatic referencing and Dereferencing
// //     println!(
// //         "The area calculated usind structs methods is: {:?}",
// //         rect.area()
// //     );
// //     if rect.can_hold(&rect2) == true {
// //         println!("Area one is bigger than area 2😂");
// //     } else {
// //         println!("Area two is bigger than area one 😂");
// //     }
// // }


// // _________________________________________________________________________________________

// //  Structs and enums are the building blocks for creating new types in rust
// //  ENUMS & pattern matching
// // ___________________________________________________________________________________________


// // // enum IpAddressKind {
// // //     IPV4,
// // //     IPV6,
// // // }
// // // enum variants can also hold data

// // enum IpAddressKind {
// //     IPV4(String),
// //     IPV6(u8, u8, u8)
// // }
// // // getting started
// // // using structs to create ip addresses
// // struct IpAddr {
// //     kind: IpAddressKind,
// //     address:String,
// // }


// // fn main() {
// //     let four=IpAddressKind::IPV4(String::from("hello"));
// //     let six=IpAddressKind::IPV6;
// //     // actual ip address
// //     let localhost=IpAddr {
// //         kind:IpAddressKind::IPV4,
// //         address: String::from("127.0.0.1")
// //     }
// // }


// // fn route(ip_kind:IpAddressKind) {
    
// // }


// // option type in Rust
// // there is no null value in Rust
// // //  enum  Option<T> {
// // //         Some(T), 
// // //         None
// // //     }

// // // Option enum is use to handle null value cases 
// // // have a value that potential nbe none or not exist then 
// // // you add it to the Option enum
// // // fn main() {

// // //    let some_number=Some(5);
// // // //    type will be inferd automaticall from the value pass above
// // // let some_string=Some("some string");
// // // let absent_number=None;
// // //     // this is included in our program scope by defau
// // // }


// // // pattern mathcing in rust

// // fn main() {
// //     let five=Some(5);
// //     let six=plus_one(five);
// //     let none=plus_one(None);
// //     println!("{} matches grearly", six);
// // }

// // fn plus_one(x:Option<i32>)->Option<i32> {
// //     match x {
// //         None=>None,
// //         Some(i)=>Some(i+1)
// //     }
// // }



// // mod kitchen {
// //     pub fn cook() {
// //         println!("Cooking in the kitchen!");
// //     }
// // // :: (the "path" separator)
// //     fn wash_dishes() {
// //         println!("Washing dishes in the kitchen!");
// //     }
// // }


// // fn main() {
// //     kitchen::cook();
// // }


// // mod restaurant {
// //     pub mod kitchen {
// //         pub fn cook() {
// //             println!("Cooking in the kitchen!");
// //         }
// //     }

// //     pub mod front_of_house {
// //         pub  fn greet_customer() {
// //             println!("Welcome to the restaurant!");
// //         }
// //     }
// // }


// // fn main() {
// //     restaurant::kitchen::cook();
// //     restaurant::front_of_house::greet_customer();
// // }










// // mod kitchen;

// // fn main() {
// //     kitchen::cook();
// // }



// // mod kitchen {
// //    pub fn cook() {
// //         println!("Cooking in the kitchen!");
// //     }
// //     // This module is Private
// // }


// // fn main(){
// //     kitchen::cook();
// //     // this will cause a compile time error because cook is private
// // }



// // Critical: child module can always see into its parent, even without he pub. Privacy onlyblocks outside access, never upward access

// // mod kitchen {
// //     fn secret_recipe() {
// //         println!("garlic butter");
// //     }

// //     pub mod grill {
// //         pub fn cook_steak() {
// //             super::secret_recipe(); // Accessing the parent module's private function
// //             println!("Cooking steak with secret recipe!");
// //         }
// //     }
// // }



// // mod kitchen {

// //     fn secret_recipe() {
// //         println!("garlic buster.. i am cokking rh foood");
// //     } 

// //     pub mod private_keitchen {
// //         pub fn private_secrete(){
// //             // i want to acces the private module here
// //             super::secret_recipe();
// //             println!("after printing secrete module");
// //         } 
// //     }
// // }


// // fn main(){
// //     kitchen::private_keitchen::private_secrete();
// // }
// // use rust_course::greet;

// // fn main() {

// // }

// use rand::Rng;

// fn main() {
//     let mut rng=rand::thread_rng();
//     let n:u32=rng.gen_range(1..=100);
//     println!("{n}");
// }

// mod kitchen;

// fn main() {
//     kitchen::grill::anything_here();
//     kitchen::oven::oven_here();
// }


// stufyin external crates

// use rand::Rng;
// fn main() {
//     let mut rng=rand::thread_rng();
//     for i in (1..100) {
//           let n: u32=rng.gen_range(1..=100);
//     println!("{n}");
//     // println!("{n}");
    

//     }
//     // let n: u32=rng.gen_range(1..=100);
//     // println!("{n}");
//     // println!("{n}")
// }


use rust_course::{Task, Priority};

fn main() {
   let task1=Task::new("Finish Rust project", Priority::High, Some("Complete the Rust project by the end of the week".to_string()));
   let task2=Task::new("Buy groceries", Priority::Medium, None);

   println!("Task 1: {} (Priority: {:?}) - {:?}", task1.title, task1.priority, task1.description);
   println!("{:?}", task1.display());
}