// Exercise 5: Line Class
// ======================
// Create a Line class with start and end Point objects (composition).
// The Line class should have:
// - Default constructor (points set to 0, 0)
// - Constructor with start and end points
// - Copy constructor (Clone trait)
// - Destructor (Drop trait - optional in Rust)
// - Getters and setters for start/end points
// - ToString() function
// - Length() function using delegation to Point's distance method
//
// Use const arguments (&), const functions (&self), and pass by reference

mod point;
mod line;

use point::Point;
use line::Line;

fn main() {
    println!("Exercise 5: Line Class with Composition");
    println!("========================================\n");
    
    // Test default constructor
    println!("=== Default Constructor ===");
    let line1 = Line::default();
    println!("Default line: {}", line1);
    println!("Length: {:.2}\n", line1.length());
    
    // Test constructor with points
    println!("=== Constructor with Points ===");
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    let line2 = Line::new(p1.clone(), p2.clone());
    println!("Line from {} to {}", p1, p2);
    println!("Line: {}", line2);
    println!("Length: {:.2} (should be 5.0)\n", line2.length());
    
    // Test copy constructor (Clone)
    println!("=== Copy Constructor (Clone) ===");
    let line3 = line2.clone();
    println!("Original: {}", line2);
    println!("Clone: {}", line3);
    println!("Both have same length: {:.2}\n", line3.length());
    
    // Test getters
    println!("=== Getters ===");
    println!("Start point: {}", line2.start());
    println!("End point: {}\n", line2.end());
    
    // Test setters
    println!("=== Setters ===");
    let mut line4 = Line::default();
    println!("Before: {}", line4);
    
    let new_start = Point::new(1.0, 1.0);
    let new_end = Point::new(4.0, 5.0);
    line4.set_start(new_start);
    line4.set_end(new_end);
    
    println!("After: {}", line4);
    println!("New length: {:.2}\n", line4.length());
    
    // Test delegation - Length uses Point's distance method
    println!("=== Delegation Demo ===");
    let line5 = Line::new(
        Point::new(0.0, 0.0),
        Point::new(5.0, 12.0)
    );
    println!("Line: {}", line5);
    println!("Length (using Point::distance): {:.2}", line5.length());
    println!("(This is a 5-12-13 right triangle)\n");
    
    // Test const correctness
    println!("=== Const Correctness ===");
    print_line_info(&line5);  // Pass immutable reference
    
    // Demonstrate composition
    println!("\n=== Composition Demonstration ===");
    println!("Line contains two Point objects:");
    println!("  - Not inherited from Point");
    println!("  - HAS-A relationship, not IS-A");
    println!("  - Delegates distance calculation to Point");
}

// Function taking immutable reference (const in C++)
fn print_line_info(line: &Line) {
    println!("Line info (immutable access):");
    println!("  {}", line);
    println!("  Start: {}", line.start());
    println!("  End: {}", line.end());
    println!("  Length: {:.2}", line.length());
}
