// Exercise 3: Constructors as conversion operators
// ===============================================
// In C++: Constructor with single argument enables implicit conversion
// In Rust: We use From/Into traits and explicit PartialEq implementations

mod point;
mod line;
mod circle;

use point::Point;
use line::Line;
use circle::Circle;

fn main() {
    println!("Level 4, Section 2.5, Exercise 3: Constructors as Conversion Operators");
    println!("=======================================================================\n");

    // Test the single-argument constructor equivalent
    println!("=== Single Value Constructor ===");
    let p1 = Point::new(1.0, 1.0);
    let p2 = Point::from_single_value(2.5);  // Both x and y set to 2.5
    let p3 = Point::from(3.0);               // Using From trait
    
    println!("p1 (regular constructor): {}", p1);
    println!("p2 (from single value): {}", p2);
    println!("p3 (using From trait): {}", p3);
    
    // Demonstrate implicit conversion concept in C++ vs Rust
    println!("\n=== C++ Implicit Conversion Simulation ===");
    
    // In C++, this would work with implicit conversion:
    // if (p==1.0) cout<<"Equal!"<<endl;
    
    // In Rust, we need explicit trait implementations
    println!("Testing point comparisons...");
    
    let point = Point::new(1.0, 1.0);
    println!("point: {}", point);
    
    // This works because we implement PartialEq<f64> for Point
    if point == 1.0 {
        println!("Point equals 1.0 (using PartialEq<f64>)!");
    } else {
        println!("Point does not equal 1.0");
    }
    
    // Test with different values
    let point2 = Point::new(2.5, 2.5);
    if point2 == 2.5 {
        println!("Point(2.5, 2.5) equals 2.5!");
    } else {
        println!("Point(2.5, 2.5) does not equal 2.5");
    }
    
    // Test inequality
    if point != 2.0 {
        println!("Point(1.0, 1.0) is not equal to 2.0");
    }
    
    // Demonstrate explicit conversion (like explicit cast in C++)
    println!("\n=== Explicit Conversion ===");
    
    // Equivalent to C++: if (p==(Point)1.0)
    if point == Point::from(1.0) {
        println!("Point equals explicitly converted 1.0!");
    }
    
    // Using Into trait (compiler infers the target type)
    let converted_point: Point = 1.5.into();
    println!("Converted point: {}", converted_point);
    
    // Demonstrate different conversion methods
    println!("\n=== Different Conversion Methods ===");
    
    let value = 4.0;
    
    // Method 1: Direct constructor call
    let p_method1 = Point::from_single_value(value);
    
    // Method 2: Using From trait
    let p_method2 = Point::from(value);
    
    // Method 3: Using Into trait
    let p_method3: Point = value.into();
    
    // Method 4: Explicit constructor
    let p_method4 = Point::new(value, value);
    
    println!("Method 1 (direct): {}", p_method1);
    println!("Method 2 (From): {}", p_method2);
    println!("Method 3 (Into): {}", p_method3);
    println!("Method 4 (explicit): {}", p_method4);
    
    // All should be equal
    assert_eq!(p_method1, p_method2);
    assert_eq!(p_method2, p_method3);
    assert_eq!(p_method3, p_method4);
    println!("All conversion methods produce equal results!");
    
    // Test with Line and Circle using converted points
    println!("\n=== Using Conversions with Line and Circle ===");
    
    let start: Point = 0.0.into();  // Convert 0.0 to Point(0.0, 0.0)
    let end: Point = 5.0.into();    // Convert 5.0 to Point(5.0, 5.0)
    
    let line = Line::new(start, end);
    println!("Line from converted points: {}", line);
    println!("Length: {:.2}", line.length());
    
    let center: Point = 1.0.into();
    let circle = Circle::new(center, 2.0);
    println!("Circle with converted center: {}", circle);
    
    // Demonstrate the "explicit" concept
    println!("\n=== Explicit vs Implicit Concept ===");
    
    // This compiles because we implemented PartialEq<f64>
    let test_point = Point::new(3.0, 3.0);
    if test_point == 3.0 {
        println!("✓ Implicit comparison works (via PartialEq<f64>)");
    }
    
    // But we can also be explicit
    if test_point == Point::from(3.0) {
        println!("✓ Explicit conversion also works");
    }
    
    // Show what would happen if we didn't have PartialEq<f64>
    println!("\nNote: In Rust, comparisons like 'point == 3.0' only work");
    println!("because we explicitly implemented PartialEq<f64> for Point.");
    println!("Without this implementation, you'd get a compile error,");
    println!("similar to C++'s 'explicit' keyword effect.");
    
    // Demonstrate type safety
    println!("\n=== Type Safety Demonstration ===");
    
    // These all work due to our trait implementations
    let point_from_int: Point = 5.into();        // i32 -> Point (if we implement From<i32>)
    let point_from_float: Point = 5.5.into();    // f64 -> Point
    
    println!("From integer: {}", point_from_int);
    println!("From float: {}", point_from_float);
    
    // Chain operations using conversions
    let result = Point::from(2.0) + Point::from(3.0);
    println!("Point::from(2.0) + Point::from(3.0) = {}", result);
    
    // Using in mathematical operations
    let scaled = Point::from(1.0) * 4.0;
    println!("Point::from(1.0) * 4.0 = {}", scaled);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_value_constructor() {
        let p = Point::from_single_value(5.0);
        assert_eq!(p.x(), 5.0);
        assert_eq!(p.y(), 5.0);
    }

    #[test]
    fn test_from_trait() {
        let p: Point = 3.5.into();
        assert_eq!(p, Point::new(3.5, 3.5));
        
        let p2 = Point::from(2.0);
        assert_eq!(p2, Point::new(2.0, 2.0));
    }

    #[test]
    fn test_partial_eq_with_f64() {
        let p = Point::new(2.5, 2.5);
        assert!(p == 2.5);
        assert!(p != 3.0);
        
        let p2 = Point::new(1.0, 2.0);
        assert!(p2 != 1.0);  // Because y != 1.0
    }

    #[test]
    fn test_explicit_conversion() {
        let p = Point::new(4.0, 4.0);
        let converted = Point::from(4.0);
        assert_eq!(p, converted);
    }

    #[test]
    fn test_conversion_methods_equivalence() {
        let value = 7.5;
        
        let p1 = Point::from_single_value(value);
        let p2 = Point::from(value);
        let p3: Point = value.into();
        let p4 = Point::new(value, value);
        
        assert_eq!(p1, p2);
        assert_eq!(p2, p3);
        assert_eq!(p3, p4);
    }

    #[test]
    fn test_chaining_with_conversions() {
        let result = Point::from(1.0) + Point::from(2.0);
        assert_eq!(result, Point::new(3.0, 3.0));
        
        let scaled = Point::from(2.0) * 1.5;
        assert_eq!(scaled, Point::new(3.0, 3.0));
    }
}
