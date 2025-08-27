// Circle class with Display trait implementation
// ==============================================
// In C++: ostream& operator << (ostream& os, const Circle& circle);
// In Rust: impl std::fmt::Display for Circle
//
// The Display trait allows Circle objects to be printed with println!("{}", circle).

use crate::point::Point;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Circle {
    center: Point,
    radius: f64,
}

#[allow(dead_code)]
impl Circle {
    // Constructor with center point and radius
    pub fn new(center: Point, radius: f64) -> Self {
        Circle { center, radius }
    }
    
    // Default constructor - unit circle at origin
    pub fn default() -> Self {
        Circle::new(Point::default(), 1.0)
    }
    
    // Constructor for unit circle at origin
    pub fn unit_circle() -> Self {
        Circle::new(Point::default(), 1.0)
    }
    
    // Constructor for circle at origin with given radius
    pub fn at_origin(radius: f64) -> Self {
        Circle::new(Point::default(), radius)
    }
    
    // Getters
    pub fn center(&self) -> &Point {
        &self.center
    }
    
    pub fn radius(&self) -> f64 {
        self.radius
    }
    
    // Setters
    pub fn set_center(&mut self, center: Point) {
        self.center = center;
    }
    
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
    
    // Mathematical functions
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }
    
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
    
    // Check if a point is inside the circle
    pub fn contains_point(&self, point: &Point) -> bool {
        self.center.distance(point) <= self.radius
    }
    
    // Check if a point is on the circle boundary (within epsilon)
    pub fn point_on_boundary(&self, point: &Point) -> bool {
        const EPSILON: f64 = 1e-10;
        (self.center.distance(point) - self.radius).abs() < EPSILON
    }
    
    // Get point on circle at given angle (in radians)
    pub fn point_at_angle(&self, angle: f64) -> Point {
        let x = self.center.x() + self.radius * angle.cos();
        let y = self.center.y() + self.radius * angle.sin();
        Point::new(x, y)
    }
    
    // Move circle by a vector (using Point operators!)
    pub fn translate(&self, offset: Point) -> Circle {
        Circle::new(self.center + offset, self.radius)
    }
    
    // Scale circle by a factor
    pub fn scale(&self, factor: f64) -> Circle {
        Circle::new(self.center, self.radius * factor)
    }
    
    // ToString method (equivalent to C++ ToString())
    pub fn to_string_custom(&self) -> String {
        format!("Circle[center: {}, radius: {:.2}]", 
                self.center.to_string_custom(), 
                self.radius)
    }
    
    // ToString with mathematical properties
    pub fn to_string_with_properties(&self) -> String {
        format!(
            "Circle[center: {}, radius: {:.2}, area: {:.2}, circumference: {:.2}]", 
            self.center.to_string_custom(), 
            self.radius,
            self.area(),
            self.circumference()
        )
    }
}

// Display trait implementation (equivalent to C++ ostream << operator)
// ===================================================================
// In C++:
//   ostream& operator << (ostream& os, const Circle& circle) {
//       os << circle.ToString();  // Using ToString() method
//       return os;
//   }
//
// In Rust, we implement std::fmt::Display:
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // {:#} format - show more details including mathematical properties
            write!(f, "Circle[center: {}, radius: {:.2}, area: {:.2}, circumference: {:.2}]",
                   self.center, self.radius, self.area(), self.circumference())
        } else if let Some(precision) = f.precision() {
            // Custom precision for radius
            write!(f, "Circle[center: {}, radius: {:.prec$}]",
                   self.center, self.radius, prec = precision)
        } else {
            // Default format - use ToString method
            write!(f, "{}", self.to_string_custom())
        }
    }
}

// Additional formatting for different use cases

// LowerExp for scientific notation
impl fmt::LowerExp for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle[center: {:e}, radius: {:e}]", self.center, self.radius)
    }
}

// Custom Debug format with extensive information
impl fmt::Debug for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Circle")
            .field("center", &self.center)
            .field("radius", &self.radius)
            .field("diameter", &self.diameter())
            .field("area", &self.area())
            .field("circumference", &self.circumference())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let center = Point::new(1.0, 2.0);
        let circle = Circle::new(center, 5.0);
        
        assert_eq!(*circle.center(), center);
        assert_eq!(circle.radius(), 5.0);
    }

    #[test]
    fn test_default_constructor() {
        let circle = Circle::default();
        assert_eq!(*circle.center(), Point::default());
        assert_eq!(circle.radius(), 1.0);
    }

    #[test]
    fn test_unit_circle() {
        let circle = Circle::unit_circle();
        assert_eq!(*circle.center(), Point::new(0.0, 0.0));
        assert_eq!(circle.radius(), 1.0);
    }

    #[test]
    fn test_display_basic() {
        let circle = Circle::new(Point::new(1.5, 2.5), 3.0);
        let display = format!("{}", circle);
        
        // Should contain center point and radius
        assert!(display.contains("Circle["));
        assert!(display.contains("center:"));
        assert!(display.contains("Point(1.50, 2.50)"));
        assert!(display.contains("radius: 3.00"));
        assert!(display.contains("]"));
    }

    #[test]
    fn test_display_alternate() {
        let circle = Circle::new(Point::new(0.0, 0.0), 2.0);
        let display = format!("{:#}", circle);
        
        // Alternate format should include area and circumference
        assert!(display.contains("area:"));
        assert!(display.contains("circumference:"));
        
        // Check approximate values
        let area = std::f64::consts::PI * 4.0; // π * r²
        let circumference = 2.0 * std::f64::consts::PI * 2.0; // 2πr
        assert!(display.contains(&format!("area: {:.2}", area)));
        assert!(display.contains(&format!("circumference: {:.2}", circumference)));
    }

    #[test]
    fn test_display_with_precision() {
        let circle = Circle::new(Point::new(1.0, 1.0), 3.14159);
        
        let low_precision = format!("{:.1}", circle);
        let high_precision = format!("{:.4}", circle);
        
        assert!(low_precision.contains("radius: 3.1"));
        assert!(high_precision.contains("radius: 3.1416"));
    }

    #[test]
    fn test_mathematical_functions() {
        let circle = Circle::new(Point::new(0.0, 0.0), 5.0);
        
        assert_eq!(circle.diameter(), 10.0);
        assert_eq!(circle.area(), std::f64::consts::PI * 25.0);
        assert_eq!(circle.circumference(), 10.0 * std::f64::consts::PI);
    }

    #[test]
    fn test_to_string_methods() {
        let circle = Circle::new(Point::new(1.0, 2.0), 3.0);
        
        let basic = circle.to_string_custom();
        let with_props = circle.to_string_with_properties();
        
        assert!(basic.contains("Circle["));
        assert!(with_props.contains("area:"));
        assert!(with_props.contains("circumference:"));
    }

    #[test]
    fn test_debug_format() {
        let circle = Circle::new(Point::new(0.0, 0.0), 2.0);
        let debug = format!("{:?}", circle);
        
        // Debug should show all fields including calculated ones
        assert!(debug.contains("Circle"));
        assert!(debug.contains("center"));
        assert!(debug.contains("radius"));
        assert!(debug.contains("diameter"));
        assert!(debug.contains("area"));
        assert!(debug.contains("circumference"));
    }

    #[test]
    fn test_contains_point() {
        let circle = Circle::new(Point::new(0.0, 0.0), 5.0);
        
        // Point inside
        assert!(circle.contains_point(&Point::new(3.0, 4.0)));
        
        // Point on boundary
        assert!(circle.contains_point(&Point::new(5.0, 0.0)));
        
        // Point outside
        assert!(!circle.contains_point(&Point::new(6.0, 0.0)));
    }

    #[test]
    fn test_point_on_boundary() {
        let circle = Circle::new(Point::new(0.0, 0.0), 5.0);
        
        // Point exactly on boundary
        assert!(circle.point_on_boundary(&Point::new(5.0, 0.0)));
        assert!(circle.point_on_boundary(&Point::new(0.0, 5.0)));
        assert!(circle.point_on_boundary(&Point::new(3.0, 4.0))); // 3-4-5 triangle: distance = 5.0
        
        // Point not on boundary (inside)
        assert!(!circle.point_on_boundary(&Point::new(2.0, 2.0))); // Distance ≈ 2.83, inside circle
        assert!(!circle.point_on_boundary(&Point::new(1.0, 1.0))); // Distance ≈ 1.41, inside circle
        
        // Point not on boundary (outside)
        assert!(!circle.point_on_boundary(&Point::new(6.0, 0.0))); // Distance = 6.0, outside circle
        assert!(!circle.point_on_boundary(&Point::new(4.0, 4.0))); // Distance ≈ 5.66, outside circle
    }
    #[test]
    fn test_point_at_angle() {
        let circle = Circle::new(Point::new(0.0, 0.0), 1.0);
        
        let p0 = circle.point_at_angle(0.0);
        assert!((p0.x() - 1.0).abs() < 1e-10);
        assert!(p0.y().abs() < 1e-10);
        
        let p90 = circle.point_at_angle(std::f64::consts::PI / 2.0);
        assert!(p90.x().abs() < 1e-10);
        assert!((p90.y() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_translate() {
        let circle = Circle::new(Point::new(1.0, 1.0), 2.0);
        let offset = Point::new(3.0, 4.0);
        
        let translated = circle.translate(offset);
        assert_eq!(*translated.center(), Point::new(4.0, 5.0));
        assert_eq!(translated.radius(), 2.0);
    }

    #[test]
    fn test_scale() {
        let circle = Circle::new(Point::new(1.0, 1.0), 2.0);
        let scaled = circle.scale(1.5);
        
        assert_eq!(*scaled.center(), Point::new(1.0, 1.0));
        assert_eq!(scaled.radius(), 3.0);
    }

    #[test]
    fn test_factory_methods() {
        let unit = Circle::unit_circle();
        assert_eq!(unit.radius(), 1.0);
        assert_eq!(*unit.center(), Point::new(0.0, 0.0));
        
        let at_origin = Circle::at_origin(5.0);
        assert_eq!(at_origin.radius(), 5.0);
        assert_eq!(*at_origin.center(), Point::new(0.0, 0.0));
    }

    #[test]
    fn test_scientific_notation() {
        let circle = Circle::new(
            Point::new(1234.567, 0.0001234),
            9876.543
        );
        
        let sci_notation = format!("{:e}", circle);
        assert!(sci_notation.contains("e"));
    }

    #[test]
    fn test_zero_radius() {
        let circle = Circle::new(Point::new(1.0, 2.0), 0.0);
        assert_eq!(circle.area(), 0.0);
        assert_eq!(circle.circumference(), 0.0);
        assert_eq!(circle.diameter(), 0.0);
        
        let display = format!("{}", circle);
        assert!(display.contains("radius: 0.00"));
    }

    #[test]
    fn test_using_point_operators() {
        // Demonstrate using Point operators in Circle methods
        let center = Point::new(2.0, 2.0);
        let offset = Point::new(1.0, 1.0);
        let circle = Circle::new(center, 3.0);
        
        // Using Point addition in translate
        let moved = circle.translate(offset);
        assert_eq!(*moved.center(), Point::new(3.0, 3.0));
        
        // Using scaled center for construction
        let scaled_center = center * 0.5;
        let circle2 = Circle::new(scaled_center, 2.0);
        assert_eq!(*circle2.center(), Point::new(1.0, 1.0));
    }
}
