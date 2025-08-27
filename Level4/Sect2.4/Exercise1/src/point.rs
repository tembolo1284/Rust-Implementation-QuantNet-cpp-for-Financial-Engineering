// Point class with operator overloading
// ===================================
// Implements various operators through Rust traits:
// - std::ops::Neg for unary minus (-)
// - std::ops::Mul<f64> for scalar multiplication (*)
// - std::ops::Add for point addition (+)
// - std::cmp::PartialEq for equality comparison (==)
// - std::ops::MulAssign<f64> for compound assignment (*=)

use std::fmt;
use std::ops::{Neg, Mul, Add, MulAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
impl Point {
    // Constructor
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    // Default constructor - point at origin
    pub fn default() -> Self {
        Point::new(0.0, 0.0)
    }
    
    // Getters
    pub fn x(&self) -> f64 {
        self.x
    }
    
    pub fn y(&self) -> f64 {
        self.y
    }
    
    // Setters
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
    
    // Distance to another point
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    // Distance to origin
    pub fn distance_to_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// Implement Display for nice string representation
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({:.2}, {:.2})", self.x, self.y)
    }
}

// Operator overloading implementations

// Unary minus operator: -point
// Point operator - () const; // Negate the coordinates.
impl Neg for Point {
    type Output = Point;
    
    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

// Scalar multiplication: point * factor
// Point operator * (double factor) const; // Scale the coordinates.
impl Mul<f64> for Point {
    type Output = Point;
    
    fn mul(self, factor: f64) -> Self::Output {
        Point::new(self.x * factor, self.y * factor)
    }
}

// Point addition: point + point
// Point operator + (const Point& p) const; // Add coordinates.
impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

// Equality comparison is already provided by #[derive(PartialEq)]
// bool operator == (const Point& p) const; // Equally compare operator.
// This is automatically implemented by the derive macro above.

// Assignment operator is automatic in Rust through Copy/Move semantics
// Point& operator = (const Point& source); // Assignment operator.
// Rust handles this automatically - no explicit implementation needed.

// Compound assignment: point *= factor
// Point& operator *= (double factor); // Scale the coordinates & assign.
impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
        // In Rust, assignment operators don't return anything
        // Chaining is still possible: p *= 2.0; p *= 3.0;
    }
}

// Additional useful implementations for completeness

// Allow f64 * Point (commutative multiplication)
impl Mul<Point> for f64 {
    type Output = Point;
    
    fn mul(self, point: Point) -> Self::Output {
        point * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let p = Point::new(3.0, 4.0);
        assert_eq!(p.x(), 3.0);
        assert_eq!(p.y(), 4.0);
    }

    #[test]
    fn test_default_constructor() {
        let p = Point::default();
        assert_eq!(p.x(), 0.0);
        assert_eq!(p.y(), 0.0);
    }

    #[test]
    fn test_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance(&p2), 5.0);
    }

    #[test]
    fn test_distance_to_origin() {
        let p = Point::new(3.0, 4.0);
        assert_eq!(p.distance_to_origin(), 5.0);
    }

    #[test]
    fn test_unary_minus() {
        let p = Point::new(3.0, -4.0);
        let neg_p = -p;
        assert_eq!(neg_p.x(), -3.0);
        assert_eq!(neg_p.y(), 4.0);
    }

    #[test]
    fn test_scalar_multiplication() {
        let p = Point::new(3.0, 4.0);
        let scaled = p * 2.0;
        assert_eq!(scaled.x(), 6.0);
        assert_eq!(scaled.y(), 8.0);
        
        // Test commutative property
        let scaled2 = 2.0 * p;
        assert_eq!(scaled, scaled2);
    }

    #[test]
    fn test_point_addition() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        let sum = p1 + p2;
        assert_eq!(sum.x(), 4.0);
        assert_eq!(sum.y(), 6.0);
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
        assert_eq!(p.x(), 3.0);
        assert_eq!(p.y(), 4.5);
    }

    #[test]
    fn test_assignment() {
        let p1 = Point::new(3.0, 4.0);
        let p2 = p1; // Assignment (copy since Point implements Copy)
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_complex_expressions() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        
        let result = (p1 + p2) * 2.0;
        assert_eq!(result, Point::new(8.0, 12.0));
        
        let result2 = -(p1 * 2.0) + p2;
        assert_eq!(result2, Point::new(1.0, 0.0));
    }
}
