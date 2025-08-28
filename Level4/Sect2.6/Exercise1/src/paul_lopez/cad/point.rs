// Point class in CAD namespace - paul_lopez::cad::Point
// =====================================================

use std::fmt;
use std::ops::{Neg, Mul, Add, MulAssign};

/// 2D Point class
/// 
/// This represents a point in 2D Cartesian coordinate system
/// Located in the paul_lopez::cad namespace
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    /// Create a new point with given coordinates
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    /// Create a point at the origin (0, 0)
    pub fn default() -> Self {
        Point::new(0.0, 0.0)
    }
    
    /// Create a point with both coordinates set to the same value
    pub fn from_single_value(value: f64) -> Self {
        Point::new(value, value)
    }
    
    /// Get the x coordinate
    pub fn x(&self) -> f64 {
        self.x
    }
    
    /// Get the y coordinate
    pub fn y(&self) -> f64 {
        self.y
    }
    
    /// Set the x coordinate
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    
    /// Set the y coordinate
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
    
    /// Calculate distance to another point
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    /// Calculate distance to origin
    pub fn distance_to_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    /// Get the magnitude (distance from origin)
    pub fn magnitude(&self) -> f64 {
        self.distance_to_origin()
    }
    
    /// Translate point by given offset
    pub fn translate(&self, dx: f64, dy: f64) -> Point {
        Point::new(self.x + dx, self.y + dy)
    }
    
    /// Rotate point around origin by given angle (in radians)
    pub fn rotate(&self, angle: f64) -> Point {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Point::new(
            self.x * cos_a - self.y * sin_a,
            self.x * sin_a + self.y * cos_a
        )
    }
}

// Default implementation
impl Default for Point {
    fn default() -> Self {
        Point::new(0.0, 0.0)
    }
}

// Display implementation for pretty printing
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({:.2}, {:.2})", self.x, self.y)
    }
}

// Arithmetic operators

/// Unary negation: -point
impl Neg for Point {
    type Output = Point;
    
    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

/// Scalar multiplication: point * factor
impl Mul<f64> for Point {
    type Output = Point;
    
    fn mul(self, factor: f64) -> Self::Output {
        Point::new(self.x * factor, self.y * factor)
    }
}

/// Point addition: point1 + point2
impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

/// Compound assignment: point *= factor
impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }
}

/// Allow f64 * Point (commutative multiplication)
impl Mul<Point> for f64 {
    type Output = Point;
    
    fn mul(self, point: Point) -> Self::Output {
        point * self
    }
}

// Conversion traits
impl From<f64> for Point {
    fn from(value: f64) -> Self {
        Point::new(value, value)
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Point::new(x, y)
    }
}

// Cross-type comparisons
impl PartialEq<f64> for Point {
    fn eq(&self, other: &f64) -> bool {
        self.x == *other && self.y == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let p1 = Point::new(3.0, 4.0);
        assert_eq!(p1.x(), 3.0);
        assert_eq!(p1.y(), 4.0);
        
        let p2 = Point::default();
        assert_eq!(p2.x(), 0.0);
        assert_eq!(p2.y(), 0.0);
    }

    #[test]
    fn test_distance_calculations() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        
        assert_eq!(p1.distance(&p2), 5.0);
        assert_eq!(p2.distance_to_origin(), 5.0);
        assert_eq!(p2.magnitude(), 5.0);
    }

    #[test]
    fn test_transformations() {
        let p = Point::new(1.0, 2.0);
        
        let translated = p.translate(3.0, 4.0);
        assert_eq!(translated, Point::new(4.0, 6.0));
        
        let rotated_90 = p.rotate(std::f64::consts::PI / 2.0);
        assert!((rotated_90.x() - (-2.0)).abs() < 1e-10);
        assert!((rotated_90.y() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_arithmetic_operators() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        
        let sum = p1 + p2;
        assert_eq!(sum, Point::new(4.0, 6.0));
        
        let scaled = p1 * 2.0;
        assert_eq!(scaled, Point::new(2.0, 4.0));
        
        let negated = -p1;
        assert_eq!(negated, Point::new(-1.0, -2.0));
    }

    #[test]
    fn test_conversions() {
        let p1: Point = 5.0.into();
        assert_eq!(p1, Point::new(5.0, 5.0));
        
        let p2: Point = (3.0, 4.0).into();
        assert_eq!(p2, Point::new(3.0, 4.0));
    }
}
