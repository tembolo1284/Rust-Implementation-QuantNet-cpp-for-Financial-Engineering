// Circle class with proper assignment semantics
// =============================================
// Demonstrates proper assignment handling in Rust.
// Assignment is automatic and self-assignment is impossible in Rust.

use crate::point::Point;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
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
}

// Display implementation
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle[center: {}, radius: {:.2}]", self.center, self.radius)
    }
}

// Assignment operator considerations:
// ==================================
// In C++: Circle& operator = (const Circle& source);
// - Must handle self-assignment: if (this == &source) return *this;
// - Must return *this for chaining
// - Must perform deep copy of all members
// - With Point members, must ensure proper copying
//
// In Rust:
// - Assignment is automatic via Copy trait (for simple types) or Move semantics
// - Self-assignment is impossible due to borrowing rules
// - No explicit implementation needed
// - Point is Copy, so Circle is automatically Copy
// - Chaining happens naturally through ownership transfer

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
    fn test_mathematical_functions() {
        let circle = Circle::new(Point::new(0.0, 0.0), 5.0);
        
        assert_eq!(circle.diameter(), 10.0);
        assert_eq!(circle.area(), std::f64::consts::PI * 25.0);
        assert_eq!(circle.circumference(), 10.0 * std::f64::consts::PI);
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
    fn test_assignment() {
        let circle1 = Circle::new(Point::new(1.0, 2.0), 3.0);
        
        let circle2 = circle1; // Assignment (copy since Circle implements Copy)
        assert_eq!(circle1, circle2);
        
        // In C++, we'd worry about self-assignment: circle1 = circle1;
        // In Rust, this is impossible due to borrowing rules
        // The compiler would prevent: circle1 = circle1;
    }

    #[test]
    fn test_setters() {
        let mut circle = Circle::default();
        
        let new_center = Point::new(5.0, 5.0);
        circle.set_center(new_center);
        circle.set_radius(10.0);
        
        assert_eq!(*circle.center(), new_center);
        assert_eq!(circle.radius(), 10.0);
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
        
        // Using scaled center
        let scaled_center = center * 0.5;
        let circle2 = Circle::new(scaled_center, 2.0);
        assert_eq!(*circle2.center(), Point::new(1.0, 1.0));
    }
}
