// Level 5, Section 3.4, Exercise 1: Colon Syntax (Rust Translation)
// =================================================================
// 
// Original C++ Exercise: The colon syntax can improve the performance of constructors.
// Test by printing text in constructors, destructors and assignment operators.
// Execute: Line l; and count the number of Point constructor, destructor and assignment calls.
// Then change Line constructors to use colon syntax and compare performance.
// Apply colon syntax optimization to Point and Circle classes as well.
//
// Rust Translation: Demonstrates construction efficiency through ownership patterns.
// Shows difference between cloning vs moving, and optimized vs non-optimized construction.

mod point;
mod line;
mod circle;

use point::Point;
use line::{Line, LineOptimized};
use circle::{Circle, CircleOptimized};

fn main() {
    println!("Level 5.3.4 Exercise 1: Construction Optimization");
    println!("==================================================\n");

    // Test 1: Original Line construction (equivalent to C++ without colon syntax)
    println!("=== Test 1: Non-Optimized Line Construction ===");
    println!("Creating Line l1...");
    let l1 = Line::default();
    println!("Line created: {}\n", l1);

    // Test 2: Optimized Line construction (equivalent to C++ with colon syntax)
    println!("=== Test 2: Optimized Line Construction ===");
    println!("Creating LineOptimized l2...");
    let l2 = LineOptimized::default();
    println!("Line created: {}\n", l2);

    // Test 3: Construction with parameters - Non-optimized
    println!("=== Test 3: Non-Optimized Parameterized Construction ===");
    println!("Creating Line with specific points...");
    let p1 = Point::new(1.0, 2.0);
    let p2 = Point::new(3.0, 4.0);
    let l3 = Line::new(p1, p2);
    println!("Line created: {}\n", l3);

    // Test 4: Construction with parameters - Optimized
    println!("=== Test 4: Optimized Parameterized Construction ===");
    println!("Creating LineOptimized with specific points...");
    let l4 = LineOptimized::new(Point::new(5.0, 6.0), Point::new(7.0, 8.0));
    println!("Line created: {}\n", l4);

    // Test 5: Circle construction comparison
    println!("=== Test 5: Circle Construction Comparison ===");
    println!("Non-optimized Circle:");
    let c1 = Circle::new(Point::new(0.0, 0.0), 5.0);
    println!("Circle: {}\n", c1);
    
    println!("Optimized Circle:");
    let c2 = CircleOptimized::new(Point::new(0.0, 0.0), 5.0);
    println!("Circle: {}\n", c2);

    // Test 6: Assignment and cloning
    println!("=== Test 6: Assignment and Cloning ===");
    println!("Cloning a line...");
    let l5 = l1.clone();
    println!("Clone completed: {}\n", l5);

    // Test 7: Move semantics (Rust's equivalent to C++ move)
    println!("=== Test 7: Move Semantics ===");
    println!("Moving a line...");
    let _l6 = l4; // This is a move, not a copy
    println!("Line moved (original l4 no longer accessible)\n");
    // println!("{}", l4); // This would cause a compile error

    println!("=== Construction Performance Summary ===");
    println!("Check the output above to compare constructor/destructor calls");
    println!("between optimized and non-optimized versions.");
    println!("\nKey Rust concepts demonstrated:");
    println!("- Move semantics (ownership transfer)");
    println!("- Clone vs Copy semantics");
    println!("- Constructor optimization through direct field initialization");
    println!("- Drop trait (destructor equivalent)");
}
