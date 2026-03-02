// A slice is a reference to a contiguous sequence of elements in a collection. It allows you to view a portion of a collection without taking ownership of it, In Rust, slices are commonly used with strings and arrays. A string slice (&str) is a reference to a portion of a string, while an array slice (&[T]) is a reference to a portion of an array. Slices are useful for efficient data manipulation and can be easily created from existing collections without the need for copying data.

fn main() {
    let s = String::from("Hello, World");

    let hello=&s[0..5]; //slice from index 0 upto but not including 5
    let world=&s[6..]; //From 6 to the end

    println!("{} {}", hello, world);

    // some convienient shortcuts
    let hello=&s[..5];
    let world=&s[6..];
    let entire=&s[..];
}