// Exercise 1: Adding Operators to the Point class
// ===============================================
// Implement arithmetic and comparison operators for Point class.
// Most operators return new objects; only assignment operators modify originals.

mod point;
mod line;
mod circle;

use point::Point;
use line::Line;
use circle::Circle;

fn main() {
    println!("Level 4, Section 2.5, Exercise 1: Point Operators");
    println!("==================================================\n");

    // Test basic operators
    println!("=== Testing Point Operators ===");
    let p1 = Point::new(3.0, 4.0);
    let p2 = Point::new(1.0, 2.0);
    
    println!("p1 = {}", p1);
    println!("p2 = {}", p2);
    
    // Test unary minus (negation)
    println!("\n--- Unary Minus Operator ---");
    let neg_p1 = -p1;
    println!("-p1 = {}", neg_p1);
    
    // Test multiplication by scalar
    println!("\n--- Multiplication Operator ---");
    let scaled_p1 = p1 * 2.0;
    println!("p1 * 2.0 = {}", scaled_p1);
    
    // Test addition of points
    println!("\n--- Addition Operator ---");
    let sum = p1 + p2;
    println!("p1 + p2 = {}", sum);
    
    // Test equality
    println!("\n--- Equality Operator ---");
    let p3 = Point::new(3.0, 4.0);
    println!("p1 == p2: {}", p1 == p2);
    println!("p1 == p3: {}", p1 == p3);
    
    // Test assignment and compound assignment
    println!("\n--- Assignment and Compound Assignment ---");
    let mut p4 = p1;  // Assignment (automatic in Rust)
    println!("p4 = p1: {}", p4);
    
    p4 *= 1.5;  // Compound assignment
    println!("p4 *= 1.5: {}", p4);
    
    // Test chaining operations
    println!("\n--- Chaining Operations ---");
    let mut p5 = Point::new(1.0, 1.0);
    p5 *= 2.0;
    p5 *= 3.0;  // Chain *= operators
    println!("Chained multiplication result: {}", p5);
    
    // Complex expression
    let result = (p1 + p2) * 2.0;
    println!("(p1 + p2) * 2.0 = {}", result);
    
    let result2 = -(p1 * 0.5) + p2;
    println!("-(p1 * 0.5) + p2 = {}", result2);

    // Test Line and Circle classes with operators
    println!("\n=== Testing Line and Circle with Point Operators ===");
    
    // Create line using point arithmetic
    let start = Point::new(0.0, 0.0);
    let end = start + Point::new(3.0, 4.0);
    let line = Line::new(start, end);
    println!("Line from point arithmetic: {}", line);
    println!("Length: {:.2}", line.length());
    
    // Create circle using scaled point
    let center = Point::new(2.0, 2.0) * 0.5;  // Scale down center
    let circle = Circle::new(center, 3.0);
    println!("Circle with scaled center: {}", circle);
    
    // Test operator precedence and grouping
    println!("\n=== Operator Precedence Examples ===");
    let a = Point::new(1.0, 2.0);
    let b = Point::new(3.0, 4.0);
    let c = Point::new(5.0, 6.0);
    
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    
    // Demonstrate operator precedence
    let expr1 = a + b * 2.0;  // Should be a + (b * 2.0)
    println!("a + b * 2.0 = {}", expr1);
    
    let expr2 = (a + b) * 2.0;  // Force different precedence
    println!("(a + b) * 2.0 = {}", expr2);
    
    // Complex chaining
    let mut result = Point::new(10.0, 20.0);
    result *= 0.1;
    result = result + a;
    result = -result;
    println!("Complex operations result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unary_minus() {
        let p = Point::new(3.0, -4.0);
        let neg = -p;
        assert_eq!(neg, Point::new(-3.0, 4.0));
    }

    #[test]
    fn test_scalar_multiplication() {
        let p = Point::new(3.0, 4.0);
        let scaled = p * 2.5;
        assert_eq!(scaled, Point::new(7.5, 10.0));
    }

    #[test]
    fn test_point_addition() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        let sum = p1 + p2;
        assert_eq!(sum, Point::new(4.0, 6.0));
    }

    #[test]
    fn test_equality() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(1.0, 2.0);
        let p3 = Point::new(2.0, 1.0);
        
        assert!(p1 == p2);
        assert!(p1 != p3);
    }

    #[test]
    fn test_compound_assignment() {
        let mut p = Point::new(2.0, 3.0);
        p *= 1.5;
        assert_eq!(p, Point::new(3.0, 4.5));
    }

    #[test]
    fn test_chaining_operations() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 2.0);
        let result = (p1 + p2) * 0.5;
        assert_eq!(result, Point::new(1.5, 1.5));
    }
}
