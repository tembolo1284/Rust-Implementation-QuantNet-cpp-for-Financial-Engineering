// Exercise 2: Distance Functions
// ===============================
// Add distance functions to the Point class:
// - DistanceOrigin(): Calculate distance to origin (0, 0)
// - Distance(Point p): Calculate distance between two points
// Use std::sqrt() from cmath (Rust: .sqrt() method)

use std::io;
mod point;
use point::Point;

fn main() {
    println!("Exercise 2: Distance Functions");
    println!("==============================\n");
    
    // Get first point from user
    println!("Enter first point coordinates:");
    println!("X coordinate:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x1: f64 = input.trim().parse().expect("Please enter a valid number");
    
    println!("Y coordinate:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let y1: f64 = input.trim().parse().expect("Please enter a valid number");
    
    let p1 = Point::new(x1, y1);
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║      DISTANCE TO ORIGIN                ║");
    println!("╚════════════════════════════════════════╝");
    println!("Point: {}", p1);
    println!("Distance to origin (0, 0): {:.4}", p1.distance_origin());
    
    // Get second point from user
    println!("\nEnter second point coordinates:");
    println!("X coordinate:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x2: f64 = input.trim().parse().expect("Please enter a valid number");
    
    println!("Y coordinate:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let y2: f64 = input.trim().parse().expect("Please enter a valid number");
    
    let p2 = Point::new(x2, y2);
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║      DISTANCE BETWEEN POINTS           ║");
    println!("╚════════════════════════════════════════╝");
    println!("Point 1: {}", p1);
    println!("Point 2: {}", p2);
    println!("Distance between points: {:.4}", p1.distance(&p2));
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║         TEST CASES                     ║");
    println!("╚════════════════════════════════════════╝");
    
    // Test with known values
    let origin = Point::default();  // (0, 0)
    let unit_x = Point::new(1.0, 0.0);
    let unit_y = Point::new(0.0, 1.0);
    let pythagoras = Point::new(3.0, 4.0);
    
    println!("Origin: {}", origin);
    println!("Distance to origin: {:.4}", origin.distance_origin());
    
    println!("\nUnit X point: {}", unit_x);
    println!("Distance to origin: {:.4}", unit_x.distance_origin());
    
    println!("\nUnit Y point: {}", unit_y);
    println!("Distance to origin: {:.4}", unit_y.distance_origin());
    
    println!("\nPoint(3, 4): {}", pythagoras);
    println!("Distance to origin: {:.4} (should be 5)", pythagoras.distance_origin());
    
    println!("\nDistance from {} to {}: {:.4}", unit_x, unit_y, unit_x.distance(&unit_y));
    println!("(Should be √2 ≈ 1.4142)");
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║      C++ VS RUST COMPARISON            ║");
    println!("╚════════════════════════════════════════╝");
    
    println!("C++ signatures:");
    println!("  double DistanceOrigin() const;");
    println!("  double Distance(Point p) const;  // Pass by value (copies!)");
    println!();
    println!("Rust signatures:");
    println!("  fn distance_origin(&self) -> f64;");
    println!("  fn distance(&self, other: &Point) -> f64;  // Pass by reference");
    println!();
    println!("Key difference: Rust passes by reference to avoid copies!");
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║       PYTHAGOREAN THEOREM              ║");
    println!("╚════════════════════════════════════════╝");
    println!("Distance formula: √((x₂-x₁)² + (y₂-y₁)²)");
    println!();
    println!("Example calculation for {} to {}:", p1, p2);
    let dx = p2.get_x() - p1.get_x();
    let dy = p2.get_y() - p1.get_y();
    println!("  dx = {:.2} - {:.2} = {:.2}", p2.get_x(), p1.get_x(), dx);
    println!("  dy = {:.2} - {:.2} = {:.2}", p2.get_y(), p1.get_y(), dy);
    println!("  distance = √({:.2}² + {:.2}²)", dx, dy);
    println!("  distance = √({:.2} + {:.2})", dx*dx, dy*dy);
    println!("  distance = √{:.2}", dx*dx + dy*dy);
    println!("  distance = {:.4}", (dx*dx + dy*dy).sqrt());
}
