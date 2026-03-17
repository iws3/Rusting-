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
    // Your solution here
}

fn main() {
    //exercise_1();
    exercise_2();
    exercise_3();
}