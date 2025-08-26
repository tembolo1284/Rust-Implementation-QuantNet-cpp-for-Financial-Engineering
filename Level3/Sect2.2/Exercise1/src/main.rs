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
