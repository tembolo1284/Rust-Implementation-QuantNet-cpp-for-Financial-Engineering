use std::io;

mod point;
use point::Point;

fn main() {
    println!("Constructor and Copy Semantics Demonstration");
    println!("============================================\n");
    
    // PART 1: Basic constructor/destructor tracking
    println!("=== PART 1: Creating Points with Default Constructor ===");
    {
        println!("Creating p1 with default constructor:");
        let p1 = Point::default();
        println!("p1 created: {}\n", p1);
        
        println!("Creating p2 with default constructor:");
        let p2 = Point::default();
        println!("p2 created: {}\n", p2);
        
        println!("Points will be dropped at end of scope...");
    } // p1 and p2 dropped here
    
    println!("\n=== PART 2: Creating Points with Coordinate Constructor ===");
    
    // Get user input for coordinates
    println!("Enter x-coordinate:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let x: f64 = input.trim().parse()
        .expect("Please enter a valid number");
    
    println!("Enter y-coordinate:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let y: f64 = input.trim().parse()
        .expect("Please enter a valid number");
    
    // Use coordinate constructor
    println!("\nCreating point with coordinates ({}, {}):", x, y);
    let user_point = Point::new(x, y);
    println!("User point created: {}\n", user_point);
    
    // PART 3: Test distance function with different passing methods
    println!("=== PART 3: Distance Function Calls ===");
    
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    
    println!("\n--- Testing distance_by_value (copies the point) ---");
    println!("Before calling distance_by_value:");
    let dist1 = p1.clone().distance_by_value(p2.clone());  // Explicit clone for demonstration
    println!("Distance: {}", dist1);
    println!("After distance_by_value - originals p1/p2 still usable\n");
    
    println!("--- Testing distance_by_reference (no copy) ---");
    println!("Before calling distance_by_reference:");
    let dist2 = p1.distance_by_reference(&p2);
    println!("Distance: {}", dist2);
    println!("After distance_by_reference - no copy made\n");
    
    // PART 4: Demonstrate Clone trait (Rust's "copy constructor")
    println!("=== PART 4: Clone Demonstration ===");
    let original = Point::new(10.0, 20.0);
    println!("Creating original point:");
    
    println!("\nCloning the point (explicit copy):");
    let cloned = original.clone();
    println!("Clone complete");
    
    println!("\nBoth points exist:");
    println!("Original: {}", original);
    println!("Cloned: {}", cloned);
    
    // PART 5: Show constructor/destructor count
    println!("\n=== PART 5: Constructor/Destructor Count Summary ===");
    {
        println!("Creating 3 points in a scope:");
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 2.0);
        let p3 = p1.clone();  // Clone (like copy constructor)
        
        println!("\nCalculating distance (by value - requires clones):");
        let _ = p2.clone().distance_by_value(p3.clone());
        
        println!("\nCalculating distance (by reference - no clone):");
        let _ = p1.distance_by_reference(&p2);
        
        println!("\nEnd of scope - all points will be dropped:");

	// Show object lifecycle statistics
	// Point::print_statistics();
    }
    
    // PART 6: Comparison with C++
    println!("\n╔════════════════════════════════════════╗");
    println!("║      C++ VS RUST COMPARISON            ║");
    println!("╚════════════════════════════════════════╝");
    
    println!("\nC++ Behavior:");
    println!("• Copy constructor called automatically for pass-by-value");
    println!("• Default copy constructor provided if not defined");
    println!("• Constructor calls should match destructor calls");
    println!("• Distance(Point p) creates a copy");
    
    println!("\nRust Behavior:");
    println!("• Move semantics by default (no automatic copy)");
    println!("• Must implement Clone trait for explicit copying");
    println!("• Copy trait for cheap copies (like primitives)");
    println!("• distance(&Point) borrows, no copy needed");
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║         CONSTRUCTOR TYPES              ║");
    println!("╚════════════════════════════════════════╝");
    
    println!("\nC++ Constructors:");
    println!("• Default: Point()");
    println!("• Copy: Point(const Point& other)");
    println!("• Parameterized: Point(double x, double y)");
    println!("• Move: Point(Point&& other) [C++11]");
    
    println!("\nRust Equivalents:");
    println!("• Default: impl Default");
    println!("• Clone: impl Clone (explicit .clone())");
    println!("• Constructor: fn new(x: f64, y: f64)");
    println!("• Move: Default behavior (no copy)");
    
    println!("\nEnd of program - remaining points will be dropped");
}
