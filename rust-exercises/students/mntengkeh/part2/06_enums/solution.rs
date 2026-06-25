// ============================================
// Student: mntengkeh
// Topic: Enums (Part 2, Day 19)
// Date: 2026-03-14
// ============================================

// Exercise 1
fn exercise_1() {
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(4.0, 5.0);
    let triangle = Shape::Triangle(4.0, 5.0, 6.0);
    let square = Shape::Square(6.0);

    println!("Area of circle: {:.4}", area(&circle));
    println!("Area of rectangle: {}", area(&rectangle));
    println!("Area of triangle: {:.4}", area(&triangle));
    println!("Area of square: {}\n", area(&square));

    println!("Perimeter of circle: {:.4}", perimeter(&circle));
    println!("Perimeter of rectangle: {}", perimeter(&rectangle));
    println!("Perimeter of triangle: {}", perimeter(&triangle));
    println!("Perimeter of square: {}\n", perimeter(&square));

    println!("{}", describe(&circle));
    println!("{}", describe(&rectangle));
    println!("{}", describe(&triangle));
    println!("{}", describe(&square));


}

enum Shape {
    Circle(f64),              // radius
    Rectangle(f64, f64),      // width, height
    Triangle(f64, f64, f64),  // three sides
    Square(f64),              // side length
}

fn area(shape: &Shape) -> f64  {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Triangle(s1, s2, s3) => {
            let s = (s1 + s2 + s3) / 2.0;
            let s: f64 = s * (s - s1) * (s - s2) * (s - s3);
            s.sqrt()
        }
        Shape::Square(side) => side * side
    }
}

fn perimeter(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 2.0 * std::f64::consts::PI * radius  ,
        Shape::Rectangle(width, height) => 2.0 * (width + height),
        Shape::Triangle(s1, s2, s3) => s1 + s2 + s3,
        Shape::Square(side) => side * 4.0
    }
}

fn describe(shape: &Shape) -> String {
    match shape {
        Shape::Circle(radius) => format!("Circle with radius {}", radius),
        Shape::Rectangle(width, height) => format!("Rectangle with width {} and height {}", width, height),
        Shape::Triangle(s1, s2, s3) => format!("Triangle with sides {}, {} and {}", s1, s2, s3),
        Shape::Square(side) => format!("Square with side {}", side),
    }

}

// Exercise 2
fn exercise_2() {
    let res1 = safe_divide(3.0, 4.0);
    print_result(res1);
    let res2 = safe_divide(3.0, 0.0);
    print_result(res2);
    let res3 = safe_sqrt(3.0);
    print_result(res3);
    let res4 = safe_sqrt(-4.0);
    print_result(res4);
}

enum MathResult {
    Success(f64),
    DivisionByZero,
    NegativeSquareRoot(f64),
    Overflow,
}

fn safe_divide(a: f64, b: f64) -> MathResult {
    if b == 0.0 {
        return MathResult::DivisionByZero;
    }
    MathResult::Success(a / b)
}

fn safe_sqrt(n: f64) -> MathResult {
    if n < 0.0 {
        return MathResult::NegativeSquareRoot(n);
    }
    MathResult::Success(n.sqrt())
}

// fn safe_power(base: f64, exp: u32) -> MathResult {

// }

fn print_result(result: MathResult) {
    match result {
        MathResult::Success(a) => println!("Operation successful. Result: {}", a),
        MathResult::DivisionByZero => println!("Operation failed! Division by zero error"),
        MathResult::NegativeSquareRoot(a) => println!("Can't find the square root of a negative numeer: {}", a),
        MathResult::Overflow => todo!()
    }
}

// Exercise 3
fn exercise_3() {
    let mut vm = VendingMachine::new();
    vm.insert_money(500.0);
    vm.select_item("cola");
    vm.print_status();
    vm.collect_item();
    vm.print_status();
    vm.insert_money(500.0);
    vm.select_item("coke");
    vm.collect_item();
    vm.print_status();
    vm.insert_money(500.0);
    vm.select_item("cola");
    vm.print_status();
}
// still abit rusty... after the first insert, you can only insert money again after collecting
// ensure to collect before trying to insert again
// i'll fix this
// i've included a lot of println statements to track status

#[derive(PartialEq)]
#[derive(Debug)]
enum VendingState {
    Idle,
    HasMoney(f64),      // amount inserted
    Dispensing(String), // item being dispensed
    OutOfStock(String), // item that's out of stock
    Error(String),      // error message
}

#[derive(Debug)]
struct VendingMachine {
    state: VendingState,
    inventory: Vec<(String, f64, u32)>, // (name, price, quantity)
    total_earned: f64,
}

impl VendingMachine {
    fn new() -> VendingMachine {
        let mut vm = VendingMachine {
            state: VendingState::Idle,
            inventory: Vec::new(),
            total_earned: 0.0
        };
        vm.inventory.push((String::from("cola"), 500.0, 1));
        vm.inventory.push((String::from("coke"), 700.0, 7));
        vm.inventory.push((String::from("vimto"), 800.0, 8));
        vm
    }

    fn insert_money(&mut self, amount: f64) {
        if self.state == VendingState::Idle {
            self.state = VendingState::HasMoney(amount);
        } else {
            println!("Machine busy! Try again shortly!");
        }
    }

    fn select_item(&mut self, item_name: &str) {
        if let VendingState::HasMoney(inserted) = self.state {
            for i in &mut self.inventory {
                if i.0 == item_name.to_string() {
                    if inserted >= i.1 {
                        if i.2 > 0 {
                            println!("success");
                            i.2 -= 1;
                            self.state = VendingState::Dispensing(format!("{item_name}"));
                            self.total_earned += inserted;
                            break;
                        } else {
                            println!("we're out of stock for {item_name}");
                            self.state = VendingState::OutOfStock(format!("we're out of stock for {item_name}"));
                            break;
                        }
                    } else {
                        self.state = VendingState::Error(format!("you inserted less than required for {item_name}"));
                        println!("the amount you inserted is not enough for {item_name}!");
                        break;
                    }
                } else {
                    continue;
                }
            }
            if let VendingState::HasMoney(_) = self.state {
                println!("oos");
                self.state = VendingState::Error(String::from("failed to select item"));
            }
            
        } else {
            println!("no money");
            self.state = VendingState::Error(String::from("Can't select an item without money!"));
        }
    }

    fn collect_item(&mut self) {
        if let VendingState::Dispensing(item) = &self.state {
            println!("\nCollecting {item}\n");
        }
        
        self.state = VendingState::Idle;
    }

    fn print_status(&self) {
        println!("{:#?}", self);
    }

    fn eject_funds(&mut self) {
        println!("Refunding...");
        //refund
        self.state = VendingState::Idle;
    }
}

fn main() {
    //exercise_1();
    //exercise_2();
    exercise_3();
}