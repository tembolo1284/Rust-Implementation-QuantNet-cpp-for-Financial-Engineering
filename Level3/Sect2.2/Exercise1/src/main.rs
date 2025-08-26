// Exercise 1: Point Class
// =======================
// Create a Point class with x- and y-coordinates. This class will be extended in further exercises.
//
// Requirements:
// 
// Header File (point.rs in Rust):
// - Private members for x- and y-coordinates (fields are private by default in Rust)
// - No need for #ifndef/#define/#endif (Rust's module system prevents multiple inclusion)
//
// Public Functionality:
// - Default constructor (Default trait implementation)
// - Destructor (Drop trait - optional in Rust due to automatic memory management)
// - Getter functions for x- and y-coordinates (get_x() and get_y())
// - Setter functions for x- and y-coordinates (set_x() and set_y())
// - ToString() function that returns a string description of the point
//   (to_string() method, plus Display trait for formatting)
//
// Implementation Notes:
// - In C++: Use std::stringstream for double-to-string conversion
// - In Rust: Use format! macro or Display trait
// - Expected output format: "Point(1.5, 3.9)"
//
// Test Program Requirements (main function):
// 1. Include the point module (mod point; use point::Point;)
// 2. Ask the user for x- and y-coordinates using std::io::stdin()
// 3. Create a Point object using the default constructor
// 4. Set the coordinates entered by the user using setter functions
// 5. Print the description of the point returned by to_string()
// 6. Print the point coordinates using the getter functions
//
// C++ to Rust Translation Notes:
// - std::cin → std::io::stdin().read_line()
// - std::stringstream → format! macro
// - Class → Struct with impl block
// - Private by default → Fields are private by default in Rust

use std::io;

mod point;
use point::Point;

fn main() {
    println!("Point Class Demonstration");
    println!("=========================\n");
    
    println!("Enter the x-coordinate:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x_coord: f64 = input.trim().parse().expect("Please enter a valid number");
    
    println!("Enter the y-coordinate:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let y_coord: f64 = input.trim().parse().expect("Please enter a valid number");
    
    let mut point = Point::default();
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║         POINT OPERATIONS               ║");
    println!("╚════════════════════════════════════════╝");
    
    println!("Default point: {}", point.to_string());
    
    point.set_x(x_coord);
    point.set_y(y_coord);
    
    println!("After setting coordinates: {}", point.to_string());
    
    println!("Using getters:");
    println!("  X coordinate: {}", point.get_x());
    println!("  Y coordinate: {}", point.get_y());
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║        ADDITIONAL EXAMPLES             ║");
    println!("╚════════════════════════════════════════╝");
    
    let mut p2 = Point::new(3.14, 2.71);
    println!("Point created with new(): {}", p2);
    
    p2.set_x(5.0);
    println!("After setting x to 5.0: {}", p2);
    
    p2.set_y(-3.5);
    println!("After setting y to -3.5: {}", p2);
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║         MULTIPLE POINTS                ║");
    println!("╚════════════════════════════════════════╝");
    
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(1.0, 1.0),
        Point::new(-1.5, 2.5),
        Point::default(),
    ];
    
    for (i, p) in points.iter().enumerate() {
        println!("Point {}: {}", i + 1, p);
    }
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║      RUST VS C++ COMPARISON            ║");
    println!("╚════════════════════════════════════════╝");
    
    println!("C++ Class:");
    println!("  class Point {{");
    println!("  private:");
    println!("      double m_x;");
    println!("      double m_y;");
    println!("  public:");
    println!("      Point();");
    println!("      ~Point();");
    println!("      double GetX();");
    println!("      void SetX(double x);");
    println!("      std::string ToString();");
    println!("  }};");
    println!();
    println!("Rust Struct:");
    println!("  pub struct Point {{");
    println!("      x: f64,");
    println!("      y: f64,");
    println!("  }}");
    println!("  impl Point {{");
    println!("      pub fn new() -> Self {{ ... }}");
    println!("      pub fn get_x(&self) -> f64 {{ ... }}");
    println!("      pub fn set_x(&mut self, x: f64) {{ ... }}");
    println!("  }}");
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║        MOVE VS COPY SEMANTICS          ║");
    println!("╚════════════════════════════════════════╝");
    
    let p3 = Point::new(10.0, 20.0);
    let p4 = p3.clone();
    println!("Original: {}", p3);
    println!("Clone: {}", p4);
    println!("Note: In Rust, assignment moves by default unless Copy trait is implemented");
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║         DESTRUCTOR BEHAVIOR            ║");
    println!("╚════════════════════════════════════════╝");
    {
        let _temp_point = Point::new(99.9, 88.8);
        println!("Temporary point created: {}", _temp_point);
        println!("Point will be automatically dropped at end of scope");
    }
    println!("Temporary point has been dropped (destructor called automatically)");
}
