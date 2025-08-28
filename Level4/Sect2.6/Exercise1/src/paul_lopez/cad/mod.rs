// CAD Namespace Module - paul_lopez::cad
// =======================================
// 
// This module represents the CAD namespace, equivalent to:
// 
// C++:
//   namespace PaulLopez {
//       namespace CAD {
//           class Point;
//           class Line;
//           class Circle;
//           class Shape;
//       }
//   }
// 
// Rust:
//   mod paul_lopez {
//       pub mod cad {
//           pub struct Point;
//           pub struct Line;
//           pub struct Circle;
//           pub struct Shape;
//       }
//   }

// Declare individual class modules
mod point;
mod line;
mod circle;
mod shape;

// Re-export all classes to make them accessible from this module
// This allows: use paul_lopez::cad::Point; instead of use paul_lopez::cad::point::Point;
pub use point::Point;
pub use line::Line;
pub use circle::Circle;
pub use shape::Shape;

// CAD-specific utilities and constants
pub const PI: f64 = std::f64::consts::PI;

/// Calculate area of a rectangle
pub fn rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

/// Calculate distance between two points (utility function)
pub fn distance_between_points(p1: &Point, p2: &Point) -> f64 {
    p1.distance(p2)
}

/// Create a unit square as an array of points
pub fn unit_square_points() -> [Point; 4] {
    [
        Point::new(0.0, 0.0),
        Point::new(1.0, 0.0),
        Point::new(1.0, 1.0),
        Point::new(0.0, 1.0),
    ]
}

/// CAD module information
pub fn module_info() -> String {
    format!("CAD module - Geometric classes for 2D computer-aided design")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cad_classes_accessible() {
        // Test that all CAD classes are accessible through re-exports
        let _point = Point::new(1.0, 2.0);
        let _line = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        let _circle = Circle::new(Point::new(0.0, 0.0), 5.0);
        let _shape = Shape::new("Triangle");
    }

    #[test]
    fn test_cad_utilities() {
        assert_eq!(rectangle_area(3.0, 4.0), 12.0);
        
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(distance_between_points(&p1, &p2), 5.0);
        
        let square = unit_square_points();
        assert_eq!(square[0], Point::new(0.0, 0.0));
        assert_eq!(square[2], Point::new(1.0, 1.0));
    }
}
