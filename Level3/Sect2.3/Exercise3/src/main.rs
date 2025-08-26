// Exercise 3: Function Overloading
// =================================
// Previously you saw that there could be more than one constructor as long as the 
// input arguments are different. You can do the same for normal member functions. 
// Thus you can rename the DistanceOrigin() function to Distance(). Also you can 
// rename the SetX() and GetX() functions to just X(). The same is true for the 
// setter and getter of the y-coordinate.
//
// IMPORTANT: Rust does NOT support function overloading!
// We'll demonstrate why and show Rust alternatives.

mod point;
use point::Point;

fn main() {
    println!("Exercise 3: Function Overloading");
    println!("================================\n");
    
    println!("╔════════════════════════════════════════╗");
    println!("║  C++ vs Rust: Function Overloading    ║");
    println!("╚════════════════════════════════════════╝\n");
    
    println!("C++ SUPPORTS overloading:");
    println!("  double Distance();           // Distance to origin");
    println!("  double Distance(Point& p);   // Distance to point");
    println!("  double X();                  // Getter");
    println!("  void X(double x);            // Setter");
    println!();
    println!("Rust does NOT support overloading!");
    println!("Must use different names or patterns.\n");
    
    // Create test points
    let mut p1 = Point::new(3.0, 4.0);
    let p2 = Point::new(6.0, 8.0);
    
    println!("╔════════════════════════════════════════╗");
    println!("║  Approach 1: Different Names          ║");
    println!("╚════════════════════════════════════════╝\n");
    
    // Traditional Rust approach - clear, explicit names
    println!("Point 1: {}", p1);
    println!("Distance to origin: {:.2}", p1.distance_origin());
    println!("Distance to Point 2: {:.2}", p1.distance_to(&p2));
    
    println!("\nGetting coordinates:");
    println!("  x = {}", p1.x());
    println!("  y = {}", p1.y());
    
    println!("\nSetting coordinates:");
    p1.set_x(10.0);
    p1.set_y(12.0);
    println!("After setting: {}", p1);
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║  Approach 2: Optional Parameters       ║");
    println!("╚════════════════════════════════════════╝\n");
    
    // Using Option for distance - single function name
    println!("Distance (no param = origin): {:.2}", p1.distance(None));
    println!("Distance to Point 2: {:.2}", p1.distance(Some(&p2)));
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║  Approach 3: Method Chaining          ║");
    println!("╚════════════════════════════════════════╝\n");
    
    // Builder pattern style - returns self for chaining
    p1.with_x(1.0).with_y(2.0);
    println!("After chaining: {}", p1);
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║  Why No Overloading in Rust?          ║");
    println!("╚════════════════════════════════════════╝\n");
    
    println!("1. Explicitness: Clear what each function does");
    println!("2. Simplicity: No complex overload resolution");
    println!("3. Readability: Function names describe behavior");
    println!("4. Type inference: Would conflict with overloading");
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║  Best Practice Recommendation         ║");
    println!("╚════════════════════════════════════════╝\n");
    
    println!("Use Approach 1 (different names):");
    println!("  ✓ Most idiomatic Rust");
    println!("  ✓ Clear and explicit");
    println!("  ✓ No runtime overhead");
    println!("  ✓ Best for API design");
}
