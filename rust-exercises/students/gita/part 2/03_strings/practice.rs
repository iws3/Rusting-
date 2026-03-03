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

// Why Two String Types?
// You might wonder why Rust needs both. The answer is flexibility and efficiency. Having String lets you create and manipulate string data dynamically at runtime. Having &str lets you reference string data without copying it or taking ownership.
// Think about a function that just needs to read a string. In JavaScript, you'd write:


fn understanding_string() {
    let mut s=String::from("Hello");
    // we can modify that because its mutable and owned
    s.push_str(", world");
    println!("{}", s);
    // we can take a slic of a string to get a &str
    let s=String::from("Hello, World");
    let hello:&str=&s[0..5];
    let world:&str=&s[6..];
    println!("{} {}", hello, world);



}

// Think about a function tha just needs to read a string  in  js you just write:

// function print_message(msg) {
//     console.log(msg)
// }

// in rust

fn prrint_message(msg:&str) {
    println!("{}", msg);
}
// The function above can accept both String values and string literals, because both can be converted to 
// &str

fn main() {
    let owned=String::from("I'm owned");
    let literal="I'm a literal";

    // when you pass &owned, Rust automatically creaates a string slice that borrows the entire string..
    // 'This is called the deref coerciomn

    print_message(&owned); //Convert String to &str

    print_message(literal); //already literal


}


// String internals: Undserstanding - UTF-8
// Rust strings are valid UTF-8. This is enforced at the type level. 
// UTF-8 IS A VARIABLE-WIDTH encoding for unicode characters

// UTF-8 solves this by encoding characters using 1 to 4 bytes depending on the character. ASCII characters like 'a' or '5' take just 1 byte. Characters from other European languages might take 2 bytes. Chinese, Japanese, and Korean characters typically take 3 bytes. Emoji often take 4 bytes.
// This means you can't index into a Rust string by number the way you can in JavaScript or Python. This code won't compile:


fn string_index_learn() {
    let s=String::from("hello");
    let first=s[0]; //Errror! Strings don't support indexing
    // why: INDEXING SUGGEST YOU'LL GET A CHARACTER, BUT RUST DIESN'T KNOW IF BYTE 0 IS A COMPLETE CHARACTER OR JUST THE FIRST BYTE OF A MULTI-BYTE CHARACTE. tO HANDLE THIS SAFELY, RUST PROVIDES DIFFERENT WAYS TO ITERATE

}



fn string_iterates() {
    let s=String::from("नमस्ते");

    // iterates over bytes (you'll get individual u8 values)

    for b in s.bytes() {
        println!("{}", b);
    }

    // Iterates over Unicode scalar values (chars)

    for c in s.chars() {
        println!("{}", c);
    }

// if you need to index, convert to chars and collect to a Vec
    let chars=Vec<char>=s.chars().collect();

}