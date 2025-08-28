// Exercise 1: CAD and Container Namespaces (Module System in Rust)
// ================================================================
//
// This exercise demonstrates Rust's module system, which is equivalent to
// C++ namespaces for organizing code and avoiding name conflicts.
//
// C++ Namespace Structure:
//   namespace PaulLopez {
//       namespace CAD {
//           class Shape;
//           class Point;
//           class Line;
//           class Circle;
//       }
//       namespace Containers {
//           class Array;
//       }
//   }
//
// Rust Module Structure:
//   mod paul_lopez {
//       pub mod cad {
//           pub struct Shape;
//           pub struct Point;
//           pub struct Line;  
//           pub struct Circle;
//       }
//       pub mod containers {
//           pub struct Array;
//       }
//   }

// Module declarations - equivalent to C++ namespace includes

#![allow(dead_code)]
#![allow(unused_imports)]
mod paul_lopez;

// Different ways to access classes in namespaces/modules:

// 1. Full class name including namespace for Point used in Array class
// In Array implementation: paul_lopez::cad::Point
// (This is done in the Array struct definition)

// 2. Full namespace for Point class in main program
// paul_lopez::cad::Point

// 3. Using declaration for single class (Line)
// C++: using PaulLopez::CAD::Line;
// Rust: use paul_lopez::cad::Line;
use paul_lopez::cad::Line;

// 4. Using declaration for complete namespace (Containers)
// C++: using namespace PaulLopez::Containers;
// Rust: use paul_lopez::containers::*;
use paul_lopez::containers::*;

// 5. Creating shorter alias for YourName::CAD namespace
// C++: namespace geom = PaulLopez::CAD;
// Rust: use paul_lopez::cad as geom;
use paul_lopez::cad as geom;

fn main() {
    println!("Level 4, Section 2.6, Exercise 1: CAD and Container Namespaces");
    println!("===============================================================\n");

    // Demonstrate different ways of accessing namespaced/module classes

    println!("=== 1. Full Namespace Path ===");
    // C++: PaulLopez::CAD::Point p1(1.0, 2.0);
    // Rust: Full module path
    let p1 = paul_lopez::cad::Point::new(1.0, 2.0);
    println!("Full path point: {}", p1);

    println!("\n=== 2. Using Declaration for Single Class (Line) ===");
    // C++: using PaulLopez::CAD::Line; then Line line(...);
    // Rust: use paul_lopez::cad::Line; then Line::new(...)
    let line1 = Line::new(
        paul_lopez::cad::Point::new(0.0, 0.0),
        paul_lopez::cad::Point::new(3.0, 4.0)
    );
    println!("Line via using declaration: {}", line1);

    println!("\n=== 3. Using Declaration for Complete Namespace (Containers) ===");
    // C++: using namespace PaulLopez::Containers; then Array<Point> arr(5);
    // Rust: use paul_lopez::containers::*; then Array::with_size(5)
    let mut arr1 = Array::with_size(5); // Array is now directly accessible
    arr1.set_element(0, paul_lopez::cad::Point::new(10.0, 20.0));
    arr1.set_element(1, paul_lopez::cad::Point::new(30.0, 40.0));
    println!("Array via namespace using: size = {}", arr1.size());
    println!("  arr1[0]: {}", arr1[0]);
    println!("  arr1[1]: {}", arr1[1]);

    println!("\n=== 4. Namespace Alias (geom = PaulLopez::CAD) ===");
    // C++: namespace geom = PaulLopez::CAD; then geom::Circle circle(...);
    // Rust: use paul_lopez::cad as geom; then geom::Circle::new(...)
    let circle1 = geom::Circle::new(geom::Point::new(5.0, 5.0), 3.0);
    println!("Circle via alias: {}", circle1);
    
    // More examples with alias
    let shape1 = geom::Shape::new("Triangle");
    println!("Shape via alias: {}", shape1);

    println!("\n=== 5. Mixed Usage Examples ===");
    
    // Create points using different access methods
    let p2 = geom::Point::new(7.0, 8.0);                    // Via alias
    let p3 = paul_lopez::cad::Point::new(9.0, 10.0);        // Via full path
    
    // Create line using imported Line class with various points
    let line2 = Line::new(p2, p3);
    println!("Mixed usage line: {}", line2);

    // Create array using imported Array class
    let mut arr2 = Array::with_size(3);
    arr2[0] = p1;                                           // Point from full path
    arr2[1] = p2;                                           // Point from alias
    arr2[2] = p3;                                           // Point from full path
    
    println!("Mixed array contents:");
    for i in 0..arr2.size() {
        println!("  arr2[{}]: {}", i, arr2[i]);
    }

    println!("\n=== 6. Demonstrating Namespace Organization Benefits ===");
    
    // Show how namespaces prevent name conflicts
    println!("CAD namespace classes:");
    println!("  Point: geometric point with x, y coordinates");
    println!("  Line: geometric line between two points");
    println!("  Circle: geometric circle with center and radius");
    println!("  Shape: base geometric shape with name");
    
    println!("Containers namespace classes:");
    println!("  Array: dynamic array container for any type");
    // We could add more containers like List, Stack, etc. without conflicts
    
    // Example: if we had Point in both namespaces, they wouldn't conflict
    let cad_point = paul_lopez::cad::Point::new(1.0, 2.0);
    // let math_point = paul_lopez::math::Point::new(1.0, 2.0, 3.0); // 3D point
    println!("CAD Point: {}", cad_point);

    println!("\n=== 7. Advanced Module Usage ===");
    
    // Demonstrate module hierarchy and cross-module usage
    
    // Array of Points (shows containers using CAD types)
    let mut geometric_array = Array::with_size(4);
    geometric_array[0] = geom::Point::new(0.0, 0.0);
    geometric_array[1] = geom::Point::new(1.0, 1.0);
    geometric_array[2] = geom::Point::new(2.0, 2.0);
    geometric_array[3] = geom::Point::new(3.0, 3.0);
    
    println!("Geometric array of points:");
    for i in 0..geometric_array.size() {
        let distance = geometric_array[i].distance_to_origin();
        println!("  Point {}: {} (distance: {:.2})", i, geometric_array[i], distance);
    }
    
    // Array of Lines
    let lines = vec![
        Line::new(geom::Point::new(0.0, 0.0), geom::Point::new(1.0, 0.0)),
        Line::new(geom::Point::new(0.0, 0.0), geom::Point::new(0.0, 1.0)),
        Line::new(geom::Point::new(0.0, 0.0), geom::Point::new(1.0, 1.0)),
    ];
    
    println!("Array of lines:");
    for (i, line) in lines.iter().enumerate() {
        println!("  Line {}: {} (length: {:.2})", i, line, line.length());
    }
    
    // Array of Circles
    let circles = vec![
        geom::Circle::new(geom::Point::new(0.0, 0.0), 1.0),
        geom::Circle::new(geom::Point::new(2.0, 2.0), 1.5),
        geom::Circle::new(geom::Point::new(-1.0, 1.0), 0.5),
    ];
    
    println!("Array of circles:");
    for (i, circle) in circles.iter().enumerate() {
        println!("  Circle {}: {} (area: {:.2})", i, circle, circle.area());
    }

    println!("\n=== 8. Namespace Resolution Scenarios ===");
    
    // Show different ways to resolve the same class
    println!("Different ways to access the same Point class:");
    
    // Method 1: Full path
    let point_full = paul_lopez::cad::Point::new(1.0, 1.0);
    
    // Method 2: Via alias
    let point_alias = geom::Point::new(1.0, 1.0);
    
    // Method 3: Local use declaration
    {
        use paul_lopez::cad::Point;
        let point_local = Point::new(1.0, 1.0);
        println!("  Local use Point: {}", point_local);
    }
    
    println!("  Full path Point: {}", point_full);
    println!("  Alias Point: {}", point_alias);
    
    // All create the same type
    assert_eq!(point_full, point_alias);
    println!("✓ All access methods create the same type");

    println!("\n=== Summary ===");
    println!("✅ Demonstrated all namespace/module usage patterns:");
    println!("  • Full module paths: paul_lopez::cad::Point::new(...)");
    println!("  • Single class import: use paul_lopez::cad::Line;");
    println!("  • Namespace import: use paul_lopez::containers::*;");
    println!("  • Module aliases: use paul_lopez::cad as geom;");
    println!("  • Cross-module usage: Array<Point> from different modules");
    println!("  • Hierarchical organization: YourName::CAD::Point pattern");
    
    println!("\n🎯 Key Benefits of Rust Modules over C++ Namespaces:");
    println!("  • Compile-time module resolution");
    println!("  • Explicit visibility control (pub keyword)");
    println!("  • File-based module organization");
    println!("  • No header/implementation file separation needed");
    println!("  • Automatic dependency management");
    println!("  • Privacy boundaries enforced at compile time");
}

// Additional demonstration functions

// Function using full namespace paths
fn demonstrate_full_paths() {
    let _point = paul_lopez::cad::Point::new(1.0, 2.0);
    let _array = paul_lopez::containers::Array::with_size(5);
}

// Function using imported types
fn demonstrate_imports() {
    use paul_lopez::cad::Point;
    use paul_lopez::containers::Array;
    
    let _point = Point::new(3.0, 4.0);
    let _array = Array::with_size(3);
}

// Function using alias
fn demonstrate_alias() {
    use paul_lopez::cad as geometry;
    
    let _point = geometry::Point::new(5.0, 6.0);
    let _circle = geometry::Circle::new(geometry::Point::default(), 2.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_namespace_access() {
        let point = paul_lopez::cad::Point::new(1.0, 2.0);
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
        
        let mut array = paul_lopez::containers::Array::with_size(2);
        array[0] = point;
        assert_eq!(array[0], point);
    }

    #[test]
    fn test_imported_classes() {
        // Line is imported at module level
        let line = Line::new(
            paul_lopez::cad::Point::new(0.0, 0.0),
            paul_lopez::cad::Point::new(3.0, 4.0)
        );
        assert_eq!(line.length(), 5.0);
        
        // Array is imported via wildcard
        let array = Array::with_size(5);
        assert_eq!(array.size(), 5);
    }

    #[test]
    fn test_namespace_alias() {
        // geom is alias for paul_lopez::cad
        let point = geom::Point::new(7.0, 8.0);
        let circle = geom::Circle::new(point, 3.0);
        
        assert_eq!(circle.radius(), 3.0);
        assert_eq!(*circle.center(), point);
    }

    #[test]
    fn test_mixed_namespace_usage() {
        // Create objects using different access patterns
        let p1 = paul_lopez::cad::Point::new(1.0, 1.0);  // Full path
        let p2 = geom::Point::new(2.0, 2.0);             // Alias
        
        let line = Line::new(p1, p2);                    // Imported class
        let mut array = Array::with_size(2);            // Imported via wildcard
        
        array[0] = p1;
        array[1] = p2;
        
        assert_eq!(array[0], p1);
        assert_eq!(array[1], p2);
        assert!(line.length() > 0.0);
    }

    #[test]
    fn test_cross_module_functionality() {
        // Test that containers can hold CAD objects
        let mut point_array = Array::with_size(3);
        point_array[0] = geom::Point::new(1.0, 2.0);
        point_array[1] = geom::Point::new(3.0, 4.0);
        point_array[2] = geom::Point::new(5.0, 6.0);
        
        // Calculate total distance
        let total_distance = (0..point_array.size()-1)
            .map(|i| point_array[i].distance(&point_array[i+1]))
            .sum::<f64>();
        
        assert!(total_distance > 0.0);
    }

    #[test]
    fn test_namespace_organization() {
        // Verify that types from different modules are distinct
        use std::any::TypeId;
        
        let point_type = TypeId::of::<paul_lopez::cad::Point>();
        let array_type = TypeId::of::<paul_lopez::containers::Array>();
        
        // They should be different types from different modules
        assert_ne!(point_type, array_type);
    }
}
