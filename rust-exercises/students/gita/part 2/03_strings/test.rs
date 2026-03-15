// // // Vectors: Dynamic Arrays
// // // Now that we understand ownership and strings, let's talk about Vec, which is Rust's growable array type. Think of it as similar to JavaScript arrays or Python lists.
// // // A Vec is a collection that stores values of the same type in a contiguous block of heap memory. Unlike arrays, which have a fixed size known at compile time, vectors can grow and shrink at runtime.
// // // Here's how you create and use vectors:


// // fn main() {
// //     // Creating vectors
// //     let v: Vec<i32> = Vec::new(); // Empty vector, must specify type
// //     let v = vec![1, 2, 3, 4, 5]; // vec! macro infers type from contents
    
// //     // Adding elements
// //     let mut v = Vec::new();
// //     v.push(5); // Compiler infers v is Vec<i32> from this
// //     v.push(6);
// //     v.push(7);
// //     println!("{:?}", v); // [5, 6, 7]
    
// //     // Accessing elements - two ways
// //     let v = vec![1, 2, 3, 4, 5];
    
// //     // Method 1: using indexing (panics if index is out of bounds)
// //     let third = v[2];
// //     println!("The third element is {}", third);
    
// //     // Method 2: using get (returns Option, safer)
// //     match v.get(2) {
// //         Some(third) => println!("The third element is {}", third),
// //         None => println!("There is no third element"),
// //     }
    
// //     // Why two methods? Let me show you
// //     // let does_not_exist = v[100]; // This will panic at runtime
    
// //     match v.get(100) {
// //         Some(value) => println!("Got {}", value),
// //         None => println!("No element at index 100"), // This executes safely
// //     }
// // }


// // Ownership and Vectors
// // Vectors follow the same ownership rules as everything else in Rust:

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];
    
//     let first = &v[0]; // Immutable borrow of the vector
    
//     // v.push(6); // Error! Can't mutably bmorrow while immutably borrowed
    
//     println!("The first element is: {}", first);
    
//     // Now we can modify because first is no longer in use
//     v.push(6);
//     println!("{:?}", v);
// }

// // Why can't we push while we have a reference to an element? Because pushing might require the vector to reallocate its storage if there isn't enough capacity. If it reallocates, it moves all the data to a new location and frees the old location. Our reference first would then be pointing to freed memory, which is a use-after-free bug. Rust prevents this at compile time.



// // Iterating Over Vectors
// // You'll often need to process every element in a vector:


// fn main() {
//     let v = vec![100, 32, 57];
    
//     // Immutable iteration
//     for element in &v {
//         println!("{}", element);
//     }
    
//     // Mutable iteration
//     let mut v = vec![100, 32, 57];
//     for element in &mut v {
//         *element += 50; // Dereference to modify the value
//     }
//     println!("{:?}", v); // [150, 82, 107]
    
//     // Taking ownership (vector becomes unusable after)
//     for element in v {
//         println!("{}", element);
//     }
//     // println!("{:?}", v); // Won't compile, v was moved
// }








// // Vectors with Different Types Using Enums
// // Vectors can only store values of one type, but sometimes you want a collection of different types. In JavaScript or Python, you'd just put different types in the same array. In Rust, you use an enum:




// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

// fn main() {
//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];
    
//     for cell in &row {
//         match cell {
//             SpreadsheetCell::Int(i) => println!("Integer: {}", i),
//             SpreadsheetCell::Float(f) => println!("Float: {}", f),
//             SpreadsheetCell::Text(s) => println!("Text: {}", s),
//         }
//     }
// }


























// // Structs: Creating Custom Types
// // Structs let you create custom data types by grouping related values together. If you're coming from JavaScript, they're similar to objects but with crucial differences. If you know C, they're like C structs but with additional capabilities.
// // Here's the basic syntax:





// // Define a struct
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     // Create an instance
//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };
    
//     // Access fields with dot notation
//     println!("User email: {}", user1.email);
    
//     // To modify, the entire instance must be mutable
//     let mut user2 = User {
//         email: String::from("another@example.com"),
//         username: String::from("anotherusername456"),
//         active: true,
//         sign_in_count: 1,
//     };
    
//     user2.email = String::from("newemail@example.com");
// }



// Notice that you can't make individual fields mutable—the entire instance is either mutable or immutable. This is different from JavaScript where you can always modify object properties.


// Creating Instances From Other Instances
// Rust provides convenient syntax for creating new instances based on existing ones:


fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // Struct update syntax - use most fields from user1
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // Fill remaining fields from user1
    };
    
    // Note: user1 can no longer be used because username and email
    // were moved to user2 (String doesn't implement Copy)
    // println!("{}", user1.username); // Won't compile
    
    // But if we only copy the Copy fields:
    let user3 = User {
        email: String::from("yet@example.com"),
        username: String::from("yetanother"),
        ..user2 // Only active and sign_in_count are copied
    };
    
    // user2 is still usable because we didn't move the String fields
    println!("{}", user2.email);
}








// Tuple Structs: Structs Without Named Fields
// Sometimes you want the type safety of a struct but don't need to name the fields:
// This is useful for creating distinct types that the compiler treats as incompatible even though they have the same structure.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // Access fields by index
    println!("First component of black: {}", black.0);
    
    // Even though Color and Point have the same structure,
    // they're different types
    // let wrong: Color = origin; // Won't compile
}
















// Methods: Adding Behavior to Structs

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     // Method that borrows self
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
    
//     // Method that borrows self mutably
//     fn grow(&mut self, amount: u32) {
//         self.width += amount;
//         self.height += amount;
//     }
    
//     // Method that takes ownership of self
//     fn consume(self) -> u32 {
//         self.width * self.height
//         // Rectangle is dropped here
//     }
    
//     // Method that takes another Rectangle as a parameter
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
    
//     // Associated function (not a method - no self parameter)
//     // Called like Rectangle::square(5)
//     fn square(size: u32) -> Rectangle {
//         Rectangle {
//             width: size,
//             height: size,
//         }
//     }
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };
//     println!("Area: {}", rect1.area());
    
//     let mut rect2 = Rectangle { width: 10, height: 20 };
//     rect2.grow(5);
//     println!("New dimensions: {}x{}", rect2.width, rect2.height);
    
//     let rect3 = Rectangle { width: 40, height: 60 };
//     println!("rect3 can hold rect2: {}", rect3.can_hold(&rect2));
    
//     let square = Rectangle::square(25);
//     println!("Square area: {}", square.area());
// }

















