// // Define structs
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
//     isAdmin:bool,
//     age:i32,
// }

// fn main() {
//     let user1=User {
//         username:String::from("fonyuygita"),
//         email:String::from("fonyuy@gmail.com"),
//         active:true,
//         age:20,
//         isAdmin:false,
//         sign_in_count:1,
//     };
//     // Access fields with dot notation
//     println!("User email: {}", user1.email);
//     println!("User name: {}", user1.username);
//     println!("User active: {}", user1.active);



//     for item in [user1.username, user1.email, user1.active.to_string(), user1.age.to_string(), user1.isAdmin.to_string(), user1.sign_in_count.to_string()] {
//         println!("{}", item);
//     }

//     // Access fields with dot notation
//     // To modify, the entire instance must be mutable
//     let mut user2=User{
//         username:String::from("john_doe"),
//         email:String::from("Eric@gmail.com"),
//         active:true,
//         sign_in_count:1,
//         age:30,
//         isAdmin:true,
//     };

//     user2.email=String::from("jude@gmail.com")

    
// }

// creating instances from other instances

fn main() {
    structs_without_name_fields();
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    };
    let user1=User {
        email:String::from("gita@gmail.com"),
        username:String::from("gita"),
        active:true,
        sign_in_count:1,
    };

    // struct update syntax
    let user2=User {
        email:String::from("gita2@gmail.com"),
        ..user1  //filed the remaining fields from the user1

    };
    //user1 is no longer valid here because we used the struct update syntax and it moved the ownership of the fields to user2. If we want to use user1 again, we need to make sure that all the fields in user1 implement the Copy trait or we can clone the fields that do not implement the Copy trait before using them in user2.
    // println!("User1 email: {}", user1.email); // This will cause an error because user1's email has been moved to user2 -wont compile

// But if wse only  copy the Copy fields:
// If any non-Copy field is moved, the original struct can no longer be used. In this case, since email and username are of type String (which does not implement the Copy trait), they are moved to user2, making user1 unusable after the struct update syntax is applied. However, active and sign_in_count are of types bool and u64 respectively, which do implement the Copy trait, so they are copied to user2 without affecting user1's usability.
let user3=User {
    email:String::from("anothergita@gmail.com"),
    username:String::from("gita34"),
    ..user2 // Only active and sign_in_count are copied because they implement the Copy trait
};
println!("User2 email: {}", user2.email);

}


fn structs_without_name_fields () {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black=Color(0,0,0);
    let origin=Point(0,0,0);
    println!("Black color: ({}, {}, {})", black.0, black.1, black.2)

}
