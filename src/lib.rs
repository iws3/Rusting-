// // src/lib.rs
// pub fn greet(name: &str) -> String {
//     format!("Hello, {name}!")
// }


// pub mod tasks;
// pub use tasks::priority::Priority;
// pub use tasks::Task;


// UNDERTANDING GENERICS.TRAITS.LIFETIMES IN RUSTS
// above is to reduce code deduplication


// 1. generics

use core::num;

// scenario 1: [terrible code with repetition]
// #[derive(Debug)]
fn main() {
    let number_list=vec![34, 50, 25, 100, 65];
    println!("{:?}", number_list)
}