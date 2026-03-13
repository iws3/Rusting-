struct Rectangle {
    width:u32,
    height:u32,
}


impl Rectangle {
    // method that borrows self
    fn area(&self)->u32 {
        self.width * self.height
    }

    // methods that borrows self mutsably
    fn grow(&mut self, amount:u32) {
        self.width +=amount;
        self.height+=amount;
    }

    // methods that takes ownership of self
    fn consume(self)->u32 {
        self.width * self.height
        // Rectanfle is dropped here
    }
    // method that takes another Rectangle as a parameter
    fn can_hold(&self, other:&Rectangle)->bool {
        self.width > other.width && self.height > other.height
    }
    // Associated function (not a method - no self parameter)
    // access it as Rectangle::square(5)
    fn square(size:u32)->Rectangle {
        Rectangle {
            width:size,
            height:size,
        }
    }

}

// use  the methods here
fn main() {
    let rect1=Rectangle {width:30, height:50};
    println!("Area: {}", rect1.area());
    let mut rect2=Rectangle {width:10, height:20};
    rect2.grow(5);
    println!("New area of rect2: {}", rect2.area());
    let rect3=Rectangle {width:40, height:60};
    println!("rect3 can hold rect2: {}", rect3.can_hold(&rect2));
    let square=Rectangle::square(25);
    println!("Square area: {}", square.area());


}