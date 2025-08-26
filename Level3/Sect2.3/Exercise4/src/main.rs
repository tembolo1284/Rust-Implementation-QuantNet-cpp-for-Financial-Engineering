// Exercise 4: Const Functions
// ============================
// In C++, you need to mark functions as 'const' to call them on const objects.
// In Rust, this is handled by the borrowing system:
// - &self methods can be called on immutable references (like C++ const)
// - &mut self methods require mutable references

mod point;
use point::Point;

fn main() {
    println!("Exercise 4: Const Functions");
    println!("===========================\n");
    
    // Create an immutable point (like C++ const Point)
    let cp = Point::new(1.5, 3.9);
    println!("Created immutable point: {}", cp);
    
    println!("\n=== Trying to modify immutable point ===");
    // This would NOT compile (uncomment to see error):
    // cp.set_x(0.3);  // ERROR: cannot borrow `cp` as mutable
    println!("cp.set_x(0.3) - Would cause compiler error!");
    println!("Error: cannot borrow `cp` as mutable");
    
    println!("\n=== Reading from immutable point ===");
    // These all work because they take &self (immutable reference)
    println!("cp.x() = {}", cp.x());
    println!("cp.y() = {}", cp.y());
    println!("cp.to_string() = {}", cp.to_string());
    println!("cp.distance_origin() = {:.2}", cp.distance_origin());
    
    let p2 = Point::new(4.5, 7.9);
    println!("cp.distance_to(&p2) = {:.2}", cp.distance_to(&p2));
    
    println!("\n=== C++ vs Rust Const Correctness ===");
    println!("C++:");
    println!("  const Point cp(1.5, 3.9);");
    println!("  cp.X();        // ERROR unless X() is marked const");
    println!("  double X() const {{ return m_x; }}  // Need const keyword");
    println!();
    println!("Rust:");
    println!("  let cp = Point::new(1.5, 3.9);  // Immutable by default");
    println!("  cp.x();        // OK if x() takes &self");
    println!("  fn x(&self) -> f64 {{ self.x }}  // &self = const");
    
    println!("\n=== Mutable Point Example ===");
    let mut mp = Point::new(10.0, 20.0);
    println!("Created mutable point: {}", mp);
    
    // Can call both &self and &mut self methods
    println!("Reading x: {}", mp.x());  // &self - OK
    mp.set_x(15.0);  // &mut self - OK because mp is mut
    println!("After setting x to 15: {}", mp);
    
    println!("\n=== Method Signatures ===");
    println!("Immutable methods (like C++ const):");
    println!("  fn x(&self) -> f64");
    println!("  fn y(&self) -> f64");
    println!("  fn distance_origin(&self) -> f64");
    println!("  fn distance_to(&self, other: &Point) -> f64");
    println!("  fn to_string(&self) -> String");
    println!();
    println!("Mutable methods (like C++ non-const):");
    println!("  fn set_x(&mut self, x: f64)");
    println!("  fn set_y(&mut self, y: f64)");
    
    // Demonstrate with a function that takes immutable reference
    print_point_info(&cp);
    print_point_info(&mp);  // Can pass mutable as immutable
    
    // Demonstrate with a function that takes mutable reference
    modify_point(&mut mp);
    println!("After modification: {}", mp);
    
    // This would NOT compile:
    // modify_point(&mut cp);  // ERROR: cannot borrow `cp` as mutable
}

// Function taking immutable reference (like C++ const Point&)
fn print_point_info(p: &Point) {
    println!("\n--- Point Info (immutable access) ---");
    println!("Coordinates: ({}, {})", p.x(), p.y());
    println!("Distance to origin: {:.2}", p.distance_origin());
}

// Function taking mutable reference (like C++ Point&)
fn modify_point(p: &mut Point) {
    println!("\n--- Modifying Point ---");
    p.set_x(p.x() + 1.0);
    p.set_y(p.y() + 1.0);
    println!("Incremented both coordinates by 1");
}
