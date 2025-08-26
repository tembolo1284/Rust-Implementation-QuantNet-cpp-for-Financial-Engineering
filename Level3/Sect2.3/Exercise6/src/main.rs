// Exercise 6: Circle Class
// ========================
// Circle with center point and radius.
// Includes diameter, area, circumference calculations.

mod point;
mod circle;

use point::Point;
use circle::Circle;

fn main() {
    println!("Exercise 6: Circle Class");
    println!("========================\n");
    
    // Default constructor
    println!("=== Default Constructor ===");
    let circle1 = Circle::default();
    println!("{}", circle1);
    println!("Diameter: {:.2}", circle1.diameter());
    println!("Area: {:.2}", circle1.area());
    println!("Circumference: {:.2}\n", circle1.circumference());
    
    // Constructor with center and radius
    println!("=== Constructor with Parameters ===");
    let center = Point::new(1.0, 2.0);
    let circle2 = Circle::new(center, 5.0);
    println!("{}", circle2);
    println!("Diameter: {:.2}", circle2.diameter());
    println!("Area: {:.2}", circle2.area());
    println!("Circumference: {:.2}\n", circle2.circumference());
    
    // Copy constructor (Clone)
    println!("=== Copy Constructor (Clone) ===");
    let circle3 = circle2.clone();
    println!("Original: {}", circle2);
    println!("Clone: {}", circle3);
    
    // Getters
    println!("\n=== Getters ===");
    println!("Center: {}", circle2.center());
    println!("Radius: {}", circle2.radius());
    
    // Setters
    println!("\n=== Setters ===");
    let mut circle4 = Circle::default();
    println!("Before: {}", circle4);
    circle4.set_center(Point::new(10.0, 10.0));
    circle4.set_radius(3.0);
    println!("After: {}", circle4);
    
    // Unit circle test
    println!("\n=== Unit Circle Test ===");
    let unit_circle = Circle::new(Point::new(0.0, 0.0), 1.0);
    println!("{}", unit_circle);
    println!("Diameter: {:.2}", unit_circle.diameter());
    println!("Area: {:.6} (should be π)", unit_circle.area());
    println!("Circumference: {:.6} (should be 2π)", unit_circle.circumference());
}
