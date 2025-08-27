// Exercise 2: Ostream << Operator (Display trait in Rust)
// ======================================================
// In C++: ostream& operator << (ostream& os, const Point& p);
// In Rust: impl std::fmt::Display for Point { ... }
// 
// The Display trait in Rust is equivalent to C++'s ostream << operator.
// It allows objects to be printed with println!("{}", obj) and format!("{}", obj).

mod point;
mod line;
mod circle;

use point::Point;
use line::Line;
use circle::Circle;

fn main() {
    println!("Level 4, Section 2.5, Exercise 2: Display Trait (ostream << equivalent)");
    println!("=======================================================================\n");

    // Test Point Display
    println!("=== Point Display Tests ===");
    let p1 = Point::new(3.14159, 2.71828);
    let p2 = Point::new(-1.0, -2.5);
    let p3 = Point::default();
    
    // Basic display (equivalent to C++: cout << p1;)
    println!("p1: {}", p1);
    println!("p2: {}", p2);
    println!("p3: {}", p3);
    
    // Different formatting options
    println!("\n--- Different Formatting Options ---");
    println!("Default format: {}", p1);
    println!("Debug format: {:?}", p1);
    println!("High precision: {:.6}", p1);
    println!("Low precision: {:.1}", p1);
    
    // Test Line Display
    println!("\n=== Line Display Tests ===");
    let line1 = Line::new(Point::new(0.0, 0.0), Point::new(3.0, 4.0));
    let line2 = Line::new(Point::new(-1.0, -1.0), Point::new(1.0, 1.0));
    let line3 = Line::default();
    
    println!("line1: {}", line1);
    println!("line2: {}", line2);
    println!("line3: {}", line3);
    println!("line1 length: {:.2}", line1.length());
    
    // Test Circle Display
    println!("\n=== Circle Display Tests ===");
    let circle1 = Circle::new(Point::new(0.0, 0.0), 5.0);
    let circle2 = Circle::new(Point::new(1.0, 1.0), 2.5);
    let circle3 = Circle::unit_circle();
    
    println!("circle1: {}", circle1);
    println!("circle2: {}", circle2);
    println!("circle3: {}", circle3);
    
    // Test collections of objects
    println!("\n=== Collections Display ===");
    let points = vec![
        Point::new(1.0, 2.0),
        Point::new(3.0, 4.0),
        Point::new(5.0, 6.0),
    ];
    
    println!("Points in vector:");
    for (i, point) in points.iter().enumerate() {
        println!("  Point {}: {}", i, point);
    }
    
    let lines = vec![
        Line::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0)),
        Line::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0)),
        Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0)),
    ];
    
    println!("\nLines with their lengths:");
    for (i, line) in lines.iter().enumerate() {
        println!("  Line {}: {} (length: {:.2})", i, line, line.length());
    }
    
    // Test string interpolation and formatting
    println!("\n=== String Interpolation ===");
    let center = Point::new(2.0, 3.0);
    let radius = 4.0;
    let circle = Circle::new(center, radius);
    
    // Using format! macro (equivalent to stringstream in C++)
    let description = format!("A circle centered at {} with radius {:.1}", center, radius);
    println!("{}", description);
    
    let detailed = format!(
        "Circle details: {}\n  Area: {:.2}\n  Circumference: {:.2}",
        circle,
        circle.area(),
        circle.circumference()
    );
    println!("{}", detailed);
    
    // Test with operators and expressions
    println!("\n=== Display with Operators ===");
    let p_start = Point::new(1.0, 1.0);
    let p_end = Point::new(4.0, 5.0);
    let midpoint = (p_start + p_end) * 0.5;
    
    println!("Start: {}", p_start);
    println!("End: {}", p_end);
    println!("Midpoint: {}", midpoint);
    println!("Line: {}", Line::new(p_start, p_end));
    
    // Demonstrate chaining with display
    println!("\n=== Method Chaining with Display ===");
    let original = Point::new(2.0, 3.0);
    let scaled = original * 2.0;
    let negated = -scaled;
    
    println!("Original: {} -> Scaled: {} -> Negated: {}", original, scaled, negated);
    
    // Test error-like scenarios
    println!("\n=== Edge Cases ===");
    let zero_circle = Circle::new(Point::new(0.0, 0.0), 0.0);
    let tiny_line = Line::new(Point::new(1.0, 1.0), Point::new(1.0, 1.0));
    
    println!("Zero radius circle: {}", zero_circle);
    println!("Zero length line: {}", tiny_line);
    println!("Zero length line length: {:.10}", tiny_line.length());
    
    // Demonstrate the power of Rust's formatting
    println!("\n=== Advanced Formatting ===");
    let points_array = [
        Point::new(1.0, 2.0),
        Point::new(3.0, 4.0),
        Point::new(5.0, 6.0),
    ];
    
    // Print as a table
    println!("Point Table:");
    println!("  Index |     X     |     Y     ");
    println!("  ------|-----------|-----------|");
    for (i, point) in points_array.iter().enumerate() {
        println!("    {}   | {:>9.2} | {:>9.2} ", i, point.x(), point.y());
    }
    
    // Using write! and writeln! macros (like fprintf in C++)
    println!("\n=== Write Macros Demo ===");
    let mut output = String::new();
    use std::fmt::Write;
    
    writeln!(&mut output, "Generated Report:").unwrap();
    writeln!(&mut output, "================").unwrap();
    writeln!(&mut output, "Point: {}", p1).unwrap();
    writeln!(&mut output, "Line: {}", line1).unwrap();
    writeln!(&mut output, "Circle: {}", circle1).unwrap();
    
    print!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_display() {
        let point = Point::new(3.14, 2.71);
        let display_string = format!("{}", point);
        assert!(display_string.contains("3.14"));
        assert!(display_string.contains("2.71"));
    }

    #[test]
    fn test_line_display() {
        let line = Line::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));
        let display_string = format!("{}", line);
        assert!(display_string.contains("1.00"));
        assert!(display_string.contains("2.00"));
        assert!(display_string.contains("3.00"));
        assert!(display_string.contains("4.00"));
    }

    #[test]
    fn test_circle_display() {
        let circle = Circle::new(Point::new(1.0, 2.0), 3.0);
        let display_string = format!("{}", circle);
        assert!(display_string.contains("1.00"));
        assert!(display_string.contains("2.00"));
        assert!(display_string.contains("3.00"));
    }

    #[test]
    fn test_formatting_precision() {
        let point = Point::new(3.14159265, 2.71828182);
        
        let default_format = format!("{}", point);
        let high_precision = format!("{:.6}", point);
        let low_precision = format!("{:.1}", point);
        
        assert!(default_format.len() < high_precision.len());
        assert!(low_precision.len() < default_format.len());
    }

    #[test]
    fn test_debug_vs_display() {
        let point = Point::new(1.0, 2.0);
        
        let display = format!("{}", point);
        let debug = format!("{:?}", point);
        
        // Debug should contain the struct name
        assert!(debug.contains("Point"));
        // Display should be more user-friendly
        assert!(display.contains("Point"));
    }
}
