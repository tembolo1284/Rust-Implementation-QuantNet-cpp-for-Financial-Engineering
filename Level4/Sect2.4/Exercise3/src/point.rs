// Point class with conversion operators - Exercise 3
// ================================================
// Demonstrates Rust's equivalent to C++ constructor conversion operators.
// In C++: Point(double value) enables implicit conversion
// In Rust: From/Into traits + PartialEq<T> provide explicit conversion control

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
    
    // Single-value constructor (equivalent to C++ Point(double value))
    // This is the constructor that would enable implicit conversion in C++
    pub fn from_single_value(value: f64) -> Self {
        Point::new(value, value)
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
    
    // ToString method
    pub fn to_string_custom(&self) -> String {
        format!("Point({:.2}, {:.2})", self.x, self.y)
    }
    
    // ToString with custom precision
    pub fn to_string_precision(&self, precision: usize) -> String {
        format!("Point({:.prec$}, {:.prec$})", self.x, self.y, prec = precision)
    }
}

// Display trait implementation
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "Point({:.prec$}, {:.prec$})", self.x, self.y, prec = precision)
        } else {
            write!(f, "{}", self.to_string_custom())
        }
    }
}

// Operator implementations (from previous exercises)

// Unary minus operator: -point
impl Neg for Point {
    type Output = Point;
    
    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

// Scalar multiplication: point * factor
impl Mul<f64> for Point {
    type Output = Point;
    
    fn mul(self, factor: f64) -> Self::Output {
        Point::new(self.x * factor, self.y * factor)
    }
}

// Point addition: point + point
impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

// Compound assignment: point *= factor
impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }
}

// Allow f64 * Point (commutative multiplication)
impl Mul<Point> for f64 {
    type Output = Point;
    
    fn mul(self, point: Point) -> Self::Output {
        point * self
    }
}

// CONVERSION OPERATORS - The main focus of Exercise 3
// =====================================================

// From<f64> trait - Rust's equivalent to C++ constructor conversion
// This allows: Point::from(value) and let p: Point = value.into();
impl From<f64> for Point {
    /// Converts a single f64 value to a Point where both x and y equal the value.
    /// This is Rust's explicit equivalent to C++'s implicit constructor conversion.
    /// 
    /// In C++: Point p = 5.0; (if constructor is not explicit)
    /// In Rust: let p = Point::from(5.0); or let p: Point = 5.0.into();
    fn from(value: f64) -> Self {
        Point::from_single_value(value)
    }
}

// From<i32> trait for integer conversion
impl From<i32> for Point {
    fn from(value: i32) -> Self {
        Point::from_single_value(value as f64)
    }
}

// CROSS-TYPE COMPARISON - The heart of the exercise
// ================================================
// This enables the C++ behavior: if (point == 1.0)
// In C++, this works due to implicit constructor conversion of 1.0 to Point(1.0, 1.0)
// In Rust, we explicitly implement this comparison

impl PartialEq<f64> for Point {
    /// Enables comparison between Point and f64: if (point == 1.0)
    /// 
    /// In C++: 
    ///   Point p(1.0, 1.0);
    ///   if (p == 1.0) cout << "Equal!" << endl;  // Works due to implicit conversion
    /// 
    /// In Rust:
    ///   let p = Point::new(1.0, 1.0);
    ///   if p == 1.0 { println!("Equal!"); }     // Works due to this impl
    fn eq(&self, other: &f64) -> bool {
        // Point equals f64 if both coordinates equal the value
        self.x == *other && self.y == *other
    }
}

// Allow f64 == Point as well (symmetric comparison)
impl PartialEq<Point> for f64 {
    fn eq(&self, other: &Point) -> bool {
        other == self
    }
}

// Additional convenience: comparison with i32
impl PartialEq<i32> for Point {
    fn eq(&self, other: &i32) -> bool {
        let other_f64 = *other as f64;
        self.x == other_f64 && self.y == other_f64
    }
}

impl PartialEq<Point> for i32 {
    fn eq(&self, other: &Point) -> bool {
        other == self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_value_constructor() {
        let p = Point::from_single_value(3.5);
        assert_eq!(p.x(), 3.5);
        assert_eq!(p.y(), 3.5);
        assert_eq!(p, Point::new(3.5, 3.5));
    }

    #[test]
    fn test_from_f64_conversion() {
        // Test various ways to create Point from f64
        let p1 = Point::from(2.5);
        let p2: Point = 2.5.into();
        let p3 = Point::from_single_value(2.5);
        
        assert_eq!(p1, Point::new(2.5, 2.5));
        assert_eq!(p2, Point::new(2.5, 2.5));
        assert_eq!(p3, Point::new(2.5, 2.5));
        assert_eq!(p1, p2);
        assert_eq!(p2, p3);
    }

    #[test]
    fn test_from_i32_conversion() {
        let p1 = Point::from(5);
        let p2: Point = 5.into();
        
        assert_eq!(p1, Point::new(5.0, 5.0));
        assert_eq!(p2, Point::new(5.0, 5.0));
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_cpp_implicit_conversion_equivalent() {
        // This is the main test from the exercise!
        // C++: Point p(1.0, 1.0); if (p==1.0) cout<<"Equal!"<<endl;
        
        let p = Point::new(1.0, 1.0);
        
        // This should compile and return true (equivalent to C++ implicit conversion)
        assert!(p == 1.0);
        
        // Test with different values
        let p2 = Point::new(2.5, 2.5);
        assert!(p2 == 2.5);
        assert!(p2 != 1.0);
        
        // Point with different x and y should not equal single value
        let p3 = Point::new(1.0, 2.0);
        assert!(p3 != 1.0);  // Because y != 1.0
        assert!(p3 != 2.0);  // Because x != 2.0
    }

    #[test]
    fn test_symmetric_comparison() {
        let p = Point::new(3.0, 3.0);
        
        // Both directions should work
        assert!(p == 3.0);
        assert!(3.0 == p);
        
        assert!(p != 4.0);
        assert!(4.0 != p);
    }

    #[test]
    fn test_explicit_conversion() {
        // C++: if (p==(Point)1.0) cout<<"Equal!"<<endl;
        let p = Point::new(1.0, 1.0);
        
        // Explicit conversion equivalent
        assert!(p == Point::from(1.0));
        assert!(p == Point::from_single_value(1.0));
        
        let converted: Point = 1.0.into();
        assert!(p == converted);
    }

    #[test]
    fn test_integer_comparison() {
        let p = Point::new(5.0, 5.0);
        
        assert!(p == 5);
        assert!(5 == p);
        assert!(p != 6);
        assert!(6 != p);
    }

    #[test]
    fn test_conversion_with_operators() {
        // Test using conversions in expressions
        let result = Point::from(2.0) + Point::from(3.0);
        assert_eq!(result, Point::new(5.0, 5.0));
        
        let scaled = Point::from(4.0) * 0.5;
        assert_eq!(scaled, Point::new(2.0, 2.0));
        
        // Test with mixed operations
        let p1: Point = 1.5.into();
        let p2 = -p1;
        assert_eq!(p2, Point::new(-1.5, -1.5));
    }

    #[test]
    fn test_all_conversion_methods_equivalent() {
        let value = 7.25;
        
        // All these should create the same point
        let method1 = Point::from_single_value(value);
        let method2 = Point::from(value);
        let method3: Point = value.into();
        let method4 = Point::new(value, value);
        
        assert_eq!(method1, method2);
        assert_eq!(method2, method3);
        assert_eq!(method3, method4);
        
        // And all should compare equal to the original value
        assert!(method1 == value);
        assert!(method2 == value);
        assert!(method3 == value);
        assert!(method4 == value);
    }

    #[test]
    fn test_basic_operators_still_work() {
        // Ensure our conversion implementations don't break existing operators
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        
        let sum = p1 + p2;
        assert_eq!(sum, Point::new(4.0, 6.0));
        
        let scaled = p1 * 2.0;
        assert_eq!(scaled, Point::new(2.0, 4.0));
        
        let negated = -p1;
        assert_eq!(negated, Point::new(-1.0, -2.0));
        
        let mut p3 = p1;
        p3 *= 3.0;
        assert_eq!(p3, Point::new(3.0, 6.0));
    }
}
