// Level 5, Section 3.4, Exercise 2: Creating Shape Base Class (Rust Translation)
// ============================================================================
//
// Original C++ Exercise: Create inheritance hierarchy with Shape base class
// and Point/Line/Circle derived classes. Demonstrate polymorphism.
//
// Rust Translation: Uses traits and composition to achieve similar functionality
// without traditional inheritance. Demonstrates Rust's approach to OOP.

mod shape;
mod point;
mod line;
mod circle;

use shape::{Shape, ShapeImpl};
use point::Point;
use line::Line;
use circle::Circle;

fn main() {
    println!("Level 5.3.4 Exercise 2: Shape Hierarchy");
    println!("========================================\n");

    // Test 1: Create basic shapes (equivalent to C++ code)
    println!("=== Test 1: Basic Shape Creation ===");
    
    let s = ShapeImpl::new();  // Create shape (equivalent to Shape s;)
    let p = Point::new(10.0, 20.0);  // Create point
    let l = Line::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));  // Create line
    
    println!("{}", s);  // Print shape
    println!("{}", p);  // Print point  
    println!("{}", l);  // Print line
    println!();

    // Test 2: ID access (answers C++ questions)
    println!("=== Test 2: ID Access ===");
    println!("Shape ID: {}", s.id());  // ID of the shape
    println!("Point ID: {}", p.id());  // ID of the point. Does this work? YES - trait method
    println!("Line ID: {}", l.id());   // ID of the line. Does this work? YES - trait method
    println!();

    // Test 3: Polymorphism using trait objects (equivalent to Shape* sp)
    println!("=== Test 3: Polymorphism with Trait Objects ===");
    
    // Rust equivalent of: Shape* sp; sp=&p;
    let sp: &dyn Shape = &p;  // Point in a shape reference. Possible? YES - trait object
    println!("Polymorphic call: {}", sp.to_string());  // What is printed? Point's implementation!
    
    // Another way using Box<dyn Shape>
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(ShapeImpl::new()),
        Box::new(Point::new(5.0, 10.0)),
        Box::new(Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0))),
        Box::new(Circle::new(Point::new(0.0, 0.0), 3.0)),
    ];
    
    println!("Polymorphic iteration:");
    for (i, shape) in shapes.iter().enumerate() {
        println!("  Shape {}: {}", i, shape.to_string());
    }
    println!();

    // Test 4: Assignment and copying (answers the ID copying question)
    println!("=== Test 4: Assignment and Copying ===");
    
    let p2 = p.clone();
    println!("{}, ID: {}", p2, p2.id());
    // Is the ID copied? YES - because clone() copies the ShapeData field
    
    println!("Original point ID: {}", p.id());
    println!("Copied point ID: {}", p2.id());
    println!("Are IDs the same? {}", p.id() == p2.id());
    println!();

    // Test 5: Demonstrate trait object polymorphism with different behaviors
    println!("=== Test 5: Polymorphic Behavior Demonstration ===");
    
    fn print_shape_info(shape: &dyn Shape) {
        println!("Shape info: {}, ID: {}", shape, shape.id());
    }
    
    print_shape_info(&s);
    print_shape_info(&p);
    print_shape_info(&l);
    
    // Test 6: Show composition vs inheritance
    println!("\n=== Test 6: Composition vs Inheritance ===");
    println!("In C++: Point 'IS-A' Shape (inheritance)");
    println!("In Rust: Point 'HAS-A' ShapeData and 'IMPLEMENTS' Shape trait (composition)");
    
    let point = Point::new(42.0, 24.0);
    println!("Point: {}", point);
    println!("Point coordinates: ({}, {})", point.x(), point.y());
    println!("Point ID (from Shape trait): {}", point.id());

    println!("\n=== Key Differences Summary ===");
    println!("C++ Inheritance:");
    println!("  ✓ 'Is-a' relationship");
    println!("  ✓ Shared data through inheritance");
    println!("  ✗ Can lead to diamond problem");
    println!("  ✗ Tight coupling");
    println!();
    println!("Rust Composition + Traits:");
    println!("  ✓ 'Has-a' + 'Can-do' relationship");
    println!("  ✓ Explicit composition");
    println!("  ✓ No diamond problem");
    println!("  ✓ Loose coupling");
    println!("  ✓ Memory safe");
}

// Answers to the C++ questions in comments:
// 
// Q: "ID of the point. Does this work?"
// A: YES - In Rust, this works because Point implements the Shape trait,
//    which provides the id() method.
//
// Q: "ID of the line. Does this work?"  
// A: YES - Same reason as Point. Line implements Shape trait.
//
// Q: "Point in a shape variable. Possible?"
// A: YES - Using trait objects (&dyn Shape or Box<dyn Shape>), we can
//    store different types that implement the Shape trait.
//
// Q: "What is printed?"
// A: The Point's to_string() implementation is called due to dynamic dispatch.
//    Rust automatically calls the correct implementation.
//
// Q: "Is the ID copied if you do not call the base class assignment in point?"
// A: In Rust, the clone() method handles this properly. The ShapeData field
//    (containing the ID) is cloned along with the Point-specific data because
//    we implement Clone for the entire struct.
