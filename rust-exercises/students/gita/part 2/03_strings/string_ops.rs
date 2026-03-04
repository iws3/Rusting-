fn main() {
//  CREATING STRINGS
let mut s=String::new(); //Just an empty string
let s=String::from("Initialized content");
let s="initial content".to_string(); //same as above
appending_string();
concatenating_string();
replacing_part_string();
checking_contains();
spliting_String();

}


fn appending_string() {
    // appending to a string
    let mut s=String::from("fooo");
    s.push_str("bar"); //append a string slice
    s.push('!'); //append a single character
    println!("{}", s);

}


fn concatenating_string() {
    let s1=String::from("Hello, ");
    let s2=String::from("World!");
    let s3=s1+&s2;
    println!("{}", s3);
    // plus opersator uses method like: fn add(self, s: &str) -> String
    // it takes ownership of the left side and borrows the right side
}


fn multiple_concatenations(){
    let s1=String::from("tic");
    let s2=String::from("tac");
    let s3=String::from("toe");
    // multiple concatenation use  format! macro
    let s=format!("{}-{}-{}", s1, s2, s3); //Doesn't take ownership
    println!("{}", s); //tic tac toe
    println!("{}", s1); //still exist , because its ownership is nor taken
}

fn replacing_part_string() {
    let s=String::from("I like apples!");
    let new_s=s.replace("apples", "oranges");
    println!("{}", new_s); //I like ornages

}


fn checking_contains() {
    // checking if string contains something
    let s=String::from("Hello, world");
    if s.contains("world") {
        println!("found it")
    }

    else {{
        println!("not found");
    }}
}

fn spliting_String() {
    let s=String::from("hello world rust Use the Export Audit button to download a high-readability CSV containing names, institutions,
contract values, debts, and a full monthly payment audit strin");
    for part in s.split(' ') {
        println!("{}", part)
    }
}