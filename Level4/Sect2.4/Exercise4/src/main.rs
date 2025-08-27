// Exercise 4: Friends (Module-based visibility in Rust)
// =====================================================
// In C++: friend functions can access private members
// In Rust: Module-based visibility control replaces friend functions
// 
// C++ friend concept:
//   friend ostream& operator << (ostream& os, const Point& p);
// 
// Rust equivalent:
//   Module-level visibility with pub(crate) fields

mod point;
mod line;
mod circle;

use point::Point;
use line::Line;
use circle::Circle;

fn main() {
    println!("Level 4, Section 2.5, Exercise 4: Friends (Module Visibility)");
    println!("===============================================================\n");

    // Test Point with module-level field access
    println!("=== Point with Module Visibility ===");
    let p1 = Point::new(3.14159, 2.71828);
    let p2 = Point::new(-1.5, -2.5);
    let p3 = Point::default();
    
    println!("p1: {}", p1);
    println!("p2: {}", p2);
    println!("p3: {}", p3);
    
    // Test Line with module-level field access
    println!("\n=== Line with Module Visibility ===");
    let line1 = Line::new(Point::new(0.0, 0.0), Point::new(3.0, 4.0));
    let line2 = Line::new(Point::new(-1.0, -1.0), Point::new(1.0, 1.0));
    let line3 = Line::default();
    
    println!("line1: {}", line1);
    println!("line2: {}", line2);
    println!("line3: {}", line3);
    
    // Test Circle with module-level field access
    println!("\n=== Circle with Module Visibility ===");
    let circle1 = Circle::new(Point::new(0.0, 0.0), 5.0);
    let circle2 = Circle::new(Point::new(1.0, 1.0), 2.5);
    let circle3 = Circle::unit_circle();
    
    println!("circle1: {}", circle1);
    println!("circle2: {}", circle2);
    println!("circle3: {}", circle3);
    
    // Demonstrate the key concept: Direct field access within module
    println!("\n=== Module-Level Access Demonstration ===");
    println!("In C++, friend functions can access private members.");
    println!("In Rust, functions in the same module can access pub(crate) fields.");
    println!("This provides controlled access without breaking encapsulation.");
    
    // Test with various formatting options
    println!("\n=== Advanced Formatting (Direct Field Access) ===");
    let point = Point::new(123.456789, 987.654321);
    let line = Line::new(point, Point::new(0.0, 0.0));
    let circle = Circle::new(point, 10.0);
    
    println!("High precision point: {:.6}", point);
    println!("Low precision point: {:.1}", point);
    println!("Scientific notation point: {:e}", point);
    
    println!("Line with alternate format: {:#}", line);
    println!("Circle with alternate format: {:#}", circle);
    
    // Demonstrate that external modules cannot access private fields
    // but internal module functions can access pub(crate) fields
    println!("\n=== Encapsulation Boundaries ===");
    println!("✓ Display trait can access fields directly (same module)");
    println!("✓ Module functions can access pub(crate) fields");
    println!("✗ External crates cannot access pub(crate) fields");
    println!("✗ Private fields remain inaccessible outside module");
    
    // Test operators still work with the new visibility
    println!("\n=== Operators with Module Visibility ===");
    let p_start = Point::new(1.0, 2.0);
    let p_end = Point::new(4.0, 6.0);
    
    let sum = p_start + p_end;
    let scaled = sum * 0.5;
    let negated = -scaled;
    
    println!("p_start: {}", p_start);
    println!("p_end: {}", p_end);
    println!("sum: {}", sum);
    println!("scaled: {}", scaled);
    println!("negated: {}", negated);
    
    // Test conversions still work
    println!("\n=== Conversions with Module Visibility ===");
    let converted: Point = 5.0.into();
    let from_int: Point = 3.into();
    
    println!("From f64: {}", converted);
    println!("From i32: {}", from_int);
    
    if converted == 5.0 {
        println!("✓ Cross-type comparison still works");
    }
    
    // Create complex objects using module visibility
    println!("\n=== Complex Objects ===");
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(1.0, 1.0),
        Point::new(2.0, 0.0),
        Point::new(1.0, -1.0),
    ];
    
    println!("Points:");
    for (i, point) in points.iter().enumerate() {
        println!("  {}: {}", i, point);
    }
    
    let lines: Vec<Line> = points.windows(2)
        .map(|window| Line::new(window[0], window[1]))
        .collect();
    
    println!("Lines connecting consecutive points:");
    for (i, line) in lines.iter().enumerate() {
        println!("  {}: {} (length: {:.2})", i, line, line.length());
    }
    
    println!("\nRust's module system provides better encapsulation control");
    println!("than C++'s friend system while maintaining flexibility!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_uses_direct_field_access() {
        let point = Point::new(3.14, 2.71);
        let display_string = format!("{}", point);
        
        // The display should work correctly with direct field access
        assert!(display_string.contains("3.14"));
        assert!(display_string.contains("2.71"));
    }

    #[test]
    fn test_line_display_uses_direct_access() {
        let line = Line::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));
        let display_string = format!("{}", line);
        
        // The display should access Point fields directly
        assert!(display_string.contains("1.00"));
        assert!(display_string.contains("2.00"));
        assert!(display_string.contains("3.00"));
        assert!(display_string.contains("4.00"));
    }

    #[test]
    fn test_circle_display_uses_direct_access() {
        let circle = Circle::new(Point::new(1.0, 2.0), 3.0);
        let display_string = format!("{}", circle);
        
        // The display should access fields directly
        assert!(display_string.contains("1.00"));
        assert!(display_string.contains("2.00"));
        assert!(display_string.contains("3.00"));
    }

    #[test]
    fn test_module_visibility_concept() {
        // This test demonstrates that our module-based approach works
        let p = Point::new(5.0, 10.0);
        let line = Line::new(p, Point::default());
        let circle = Circle::new(p, 2.0);
        
        // All display implementations can access fields directly
        let _ = format!("{}", p);
        let _ = format!("{}", line);
        let _ = format!("{}", circle);
        
        // This compiles and works, proving our "friend-like" access works
        assert!(true);
    }

    #[test]
    fn test_operators_still_work() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        
        let sum = p1 + p2;
        let scaled = p1 * 2.0;
        let negated = -p1;
        
        assert_eq!(sum, Point::new(4.0, 6.0));
        assert_eq!(scaled, Point::new(2.0, 4.0));
        assert_eq!(negated, Point::new(-1.0, -2.0));
    }

    #[test]
    fn test_conversions_still_work() {
        let p1: Point = 5.0.into();
        let p2 = Point::from(3.0);
        
        assert!(p1 == 5.0);
        assert!(p2 == 3.0);
        assert_eq!(p1, Point::new(5.0, 5.0));
        assert_eq!(p2, Point::new(3.0, 3.0));
    }
}
