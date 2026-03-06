// Define structs
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    isAdmin:bool,
    age:i32,
}

fn main() {
    let user1=User {
        username:String::from("fonyuygita"),
        email:String::from("fonyuy@gmail.com"),
        active:true,
        age:20,
        isAdmin:false,
        sign_in_count:1,
    };
    // Access fields with dot notation
    println!("User email: {}", user1.email);
    println!("User name: {}", user1.username);
    println!("User active: {}", user1.active);

    for item in [user1.username, user1.email, user1.active.to_string(), user1.age.to_string(), user1.isAdmin.to_string(), user1.sign_in_count.to_string()] {
        println!("{}", item);
    }
    
}
