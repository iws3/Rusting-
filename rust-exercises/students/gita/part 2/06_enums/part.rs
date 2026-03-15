// understanding enums in rust
// ENUMS are one of Rust's most powerful features. They let you define a type that can be one of several variants. Each variant can have data associated with it. This is similar to "union types" in other languages, but with more safety and expressiveness.

// enum IpAddrKind {
//     V4,
//     V5,
// }

// fn main(){
//     let four=IpAddrKind::V4;
//     let siix=IpAddrKind::V6;
//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);
// }

// fn route(ip_kind:IpAddrKind) {

// }
// eNUMS CAN BE MUCH MORE POWERFUL. EACH VARIANT CAN HAVE ASSOCIATED DATA
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn main(){
    let home=IpAddr::V4(127,168, 0,1);
    let loopback=IpAddr::V6(String::from("::1"));
}

// Multiple values inside an enum variant can be named using struct-like syntax
enum Message {
    Quit,          //No data
    Move {x:i32, y:i32}, //anonymous struct
    Write(String),   //A string
    ChangeColor(i32, i32, i32), //Three i32s 
}

impl Message {
    fn call(&self) {
        // Methods body
      println!("Message called: {:?}", self);
    }
}

fn main() {
  let msg1=Messege::Quit;
  let msg2=Message::Move {x:10, y:20};
  let msg3=Message::Write(String::from("Hello,  world"));
  let msg4=Message::ChangeColor(244, 0, 0);

//   call the methods on each message
    msg1.call();
    msg2.call();
    msg3.call();
    msg4.call();
}