fn exercise_3(){
    let mut product;
    for x in 1..5{
    
        for y in 1..5 {
            product=x*y;
            // println!("{}x{}={} {:<2}", x, y, product);
            // println!("{}x{}={} {:<2}", x, y, product);
            println!("{}x{}={:<2}", x, y, product);
        }
    }
}

fn main(){
    exercise_3();
}