// Circle class in CAD namespace - paul_lopez::cad::Circle
// ======================================================
#![allow(dead_code)]

use super::Point; // Use Point from same CAD module
use std::fmt;

/// Circle class representing a circle in 2D space
/// 
/// This represents a circle defined by a center point and radius
/// Located in the paul_lopez::cad namespace
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    /// Create a new circle with given center and radius
    pub fn new(center: Point, radius: f64) -> Self {
        Circle { center, radius }
    }
    
    /// Create a unit circle at the origin
    pub fn unit_circle() -> Self {
        Circle::new(Point::default(), 1.0)
    }
    
    /// Create a circle at the origin with given radius
    pub fn at_origin(radius: f64) -> Self {
        Circle::new(Point::default(), radius)
    }
    
    /// Get the center point
    pub fn center(&self) -> &Point {
        &self.center
    }
    
    /// Get the radius
    pub fn radius(&self) -> f64 {
        self.radius
    }
    
    /// Set the center point
    pub fn set_center(&mut self, center: Point) {
        self.center = center;
    }
    
    /// Set the radius
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
    
    /// Calculate the diameter
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }
    
    /// Calculate the area
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    /// Calculate the circumference
    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
    
    /// Check if a point is inside the circle
    pub fn contains_point(&self, point: &Point) -> bool {
        self.center.distance(point) <= self.radius
    }
    
    /// Check if a point is on the circle boundary (within epsilon)
    pub fn point_on_boundary(&self, point: &Point) -> bool {
        const EPSILON: f64 = 1e-10;
        (self.center.distance(point) - self.radius).abs() < EPSILON
    }
    
    /// Get a point on the circle at given angle (in radians)
    pub fn point_at_angle(&self, angle: f64) -> Point {
        Point::new(
            self.center.x() + self.radius * angle.cos(),
            self.center.y() + self.radius * angle.sin()
        )
    }
    
    /// Move the circle by given offset
    pub fn translate(&self, dx: f64, dy: f64) -> Circle {
        Circle::new(self.center.translate(dx, dy), self.radius)
    }
    
    /// Scale the circle by given factor
    pub fn scale(&self, factor: f64) -> Circle {
        Circle::new(self.center, self.radius * factor)
    }
    
    /// Get the bounding box of the circle
    pub fn bounding_box(&self) -> (Point, Point) {
        let min_point = Point::new(
            self.center.x() - self.radius,
            self.center.y() - self.radius
        );
        let max_point = Point::new(
            self.center.x() + self.radius,
            self.center.y() + self.radius
        );
        (min_point, max_point)
    }
    
    /// Check if this circle intersects with another circle
    pub fn intersects(&self, other: &Circle) -> bool {
        let center_distance = self.center.distance(&other.center);
        let radius_sum = self.radius + other.radius;
        let radius_diff = (self.radius - other.radius).abs();
        
        center_distance >= radius_diff && center_distance <= radius_sum
    }
}

impl Default for Circle {
    fn default() -> Self {
        Circle::unit_circle()
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle[center: {}, radius: {:.2}]", self.center, self.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_creation() {
        let center = Point::new(2.0, 3.0);
        let circle = Circle::new(center, 5.0);
        
        assert_eq!(*circle.center(), center);
        assert_eq!(circle.radius(), 5.0);
    }

    #[test]
    fn test_circle_properties() {
        let circle = Circle::new(Point::new(0.0, 0.0), 3.0);
        
        assert_eq!(circle.diameter(), 6.0);
        assert_eq!(circle.area(), std::f64::consts::PI * 9.0);
        assert_eq!(circle.circumference(), 6.0 * std::f64::consts::PI);
    }

    #[test]
    fn test_point_containment() {
        let circle = Circle::new(Point::new(0.0, 0.0), 5.0);
        
        assert!(circle.contains_point(&Point::new(3.0, 4.0))); // Inside
        assert!(circle.contains_point(&Point::new(5.0, 0.0))); // On boundary
        assert!(!circle.contains_point(&Point::new(6.0, 0.0))); // Outside
    }

    #[test]
    fn test_point_at_angle() {
        let circle = Circle::unit_circle();
        
        let p0 = circle.point_at_angle(0.0);
        assert!((p0.x() - 1.0).abs() < 1e-10);
        assert!(p0.y().abs() < 1e-10);
        
        let p90 = circle.point_at_angle(std::f64::consts::PI / 2.0);
        assert!(p90.x().abs() < 1e-10);
        assert!((p90.y() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_transformations() {
        let circle = Circle::new(Point::new(1.0, 2.0), 3.0);
        
        let translated = circle.translate(4.0, 5.0);
        assert_eq!(*translated.center(), Point::new(5.0, 7.0));
        assert_eq!(translated.radius(), 3.0);
        
        let scaled = circle.scale(2.0);
        assert_eq!(*scaled.center(), Point::new(1.0, 2.0));
        assert_eq!(scaled.radius(), 6.0);
    }

    #[test]
    fn test_bounding_box() {
        let circle = Circle::new(Point::new(2.0, 3.0), 1.0);
        let (min, max) = circle.bounding_box();
        
        assert_eq!(min, Point::new(1.0, 2.0));
        assert_eq!(max, Point::new(3.0, 4.0));
    }

    #[test]
    fn test_circle_intersection() {
        let c1 = Circle::new(Point::new(0.0, 0.0), 3.0);
        let c2 = Circle::new(Point::new(4.0, 0.0), 2.0); // Touching
        let c3 = Circle::new(Point::new(10.0, 0.0), 1.0); // Far away
        
        assert!(c1.intersects(&c2));
        assert!(!c1.intersects(&c3));
    }
}
