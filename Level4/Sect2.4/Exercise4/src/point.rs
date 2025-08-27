// Point class with module-level visibility (Rust's "friend" equivalent)
// =====================================================================
// In C++: friend functions can access private members
// In Rust: pub(crate) fields allow same-crate access, maintaining encapsulation
//
// C++ friend example:
//   class Point {
//   private:
//       double x, y;
//   public:
//       friend ostream& operator << (ostream& os, const Point& p);
//   };
//
// Rust equivalent:
//   struct Point {
//       pub(crate) x: f64,  // Visible within this crate only
//       pub(crate) y: f64,  // External crates cannot access
//   }

use std::fmt;
use std::ops::{Neg, Mul, Add, MulAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    // pub(crate) = visible within this crate/module, like C++ friend access
    // External crates cannot access these fields, maintaining encapsulation
    pub(crate) x: f64,
    pub(crate) y: f64,
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
    
    // Single-value constructor
    pub fn from_single_value(value: f64) -> Self {
        Point::new(value, value)
    }
    
    // Public getters (still needed for external crate access)
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
    
    // Distance calculations
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    pub fn distance_to_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    // ToString methods (may not be needed by Display, but kept for compatibility)
    pub fn to_string_custom(&self) -> String {
        format!("Point({:.2}, {:.2})", self.x, self.y)
    }
    
    pub fn to_string_precision(&self, precision: usize) -> String {
        format!("Point({:.prec$}, {:.prec$})", self.x, self.y, prec = precision)
    }
}

// Display trait - equivalent to C++ friend ostream& operator <<
// ============================================================
// This implementation can now access x and y fields directly
// because they're pub(crate) and we're in the same crate.
// This is Rust's equivalent to C++ friend function access.

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // DIRECT FIELD ACCESS - like C++ friend function!
        // No need to call getters or to_string methods
        if let Some(precision) = f.precision() {
            write!(f, "Point({:.prec$}, {:.prec$})", 
                   self.x, self.y, prec = precision)
        } else {
            // Direct field access instead of calling to_string_custom()
            write!(f, "Point({:.2}, {:.2})", self.x, self.y)
        }
    }
}

// Additional formatting traits with direct field access
impl fmt::LowerExp for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Direct access to x and y fields
        write!(f, "Point({:e}, {:e})", self.x, self.y)
    }
}

impl fmt::UpperExp for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Direct access to x and y fields
        write!(f, "Point({:E}, {:E})", self.x, self.y)
    }
}

impl fmt::Binary for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Direct access to x and y fields
        write!(f, "Point(x_bits: {:064b}, y_bits: {:064b})", 
               self.x.to_bits(), self.y.to_bits())
    }
}

// Operator implementations (can now use direct field access)

impl Neg for Point {
    type Output = Point;
    
    fn neg(self) -> Self::Output {
        // Direct field access instead of calling getters
        Point { x: -self.x, y: -self.y }
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    
    fn mul(self, factor: f64) -> Self::Output {
        // Direct field access
        Point { x: self.x * factor, y: self.y * factor }
    }
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Self::Output {
        // Direct field access
        Point { 
            x: self.x + other.x, 
            y: self.y + other.y 
        }
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, factor: f64) {
        // Direct field modification
        self.x *= factor;
        self.y *= factor;
    }
}

// Allow f64 * Point
impl Mul<Point> for f64 {
    type Output = Point;
    
    fn mul(self, point: Point) -> Self::Output {
        point * self
    }
}

// Conversion traits (using direct field access)

impl From<f64> for Point {
    fn from(value: f64) -> Self {
        // Direct field initialization
        Point { x: value, y: value }
    }
}

impl From<i32> for Point {
    fn from(value: i32) -> Self {
        let value_f64 = value as f64;
        Point { x: value_f64, y: value_f64 }
    }
}

// Cross-type comparisons (using direct field access)

impl PartialEq<f64> for Point {
    fn eq(&self, other: &f64) -> bool {
        // Direct field access instead of calling getters
        self.x == *other && self.y == *other
    }
}

impl PartialEq<Point> for f64 {
    fn eq(&self, other: &Point) -> bool {
        other == self
    }
}

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

// Module-level helper function (can access pub(crate) fields)
// This demonstrates the "friend-like" access within the module
#[allow(dead_code)]
pub(crate) fn point_debug_info(point: &Point) -> String {
    // This function can access x and y directly because it's in the same module
    // and the fields are pub(crate). This is equivalent to C++ friend access.
    format!("Point Debug: x={:.6}, y={:.6}, distance_to_origin={:.6}", 
            point.x, point.y, point.distance_to_origin())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_field_access_in_module() {
        let p = Point::new(3.14159, 2.71828);
        
        // We can access fields directly within the module
        assert_eq!(p.x, 3.14159);
        assert_eq!(p.y, 2.71828);
        
        // This is equivalent to C++ friend function access
        let debug_info = point_debug_info(&p);
        assert!(debug_info.contains("3.14159"));
        assert!(debug_info.contains("2.71828"));
    }

    #[test]
    fn test_display_uses_direct_field_access() {
        let p = Point::new(1.23456, 6.54321);
        
        // Display implementation uses direct field access
        let display_string = format!("{}", p);
        assert!(display_string.contains("1.23"));
        assert!(display_string.contains("6.54"));
        
        // High precision format
        let precise_string = format!("{:.5}", p);
        assert!(precise_string.contains("1.23456"));
        assert!(precise_string.contains("6.54321"));
    }

    #[test]
    fn test_operators_use_direct_field_access() {
        let p1 = Point::new(2.0, 3.0);
        let p2 = Point::new(4.0, 5.0);
        
        // Operators can use direct field access
        let sum = p1 + p2;
        assert_eq!(sum.x, 6.0);
        assert_eq!(sum.y, 8.0);
        
        let scaled = p1 * 2.5;
        assert_eq!(scaled.x, 5.0);
        assert_eq!(scaled.y, 7.5);
        
        let negated = -p1;
        assert_eq!(negated.x, -2.0);
        assert_eq!(negated.y, -3.0);
    }

    #[test]
    fn test_field_modification() {
        let mut p = Point::new(1.0, 2.0);
        
        // Direct field modification within module
        p.x = 10.0;
        p.y = 20.0;
        
        assert_eq!(p.x, 10.0);
        assert_eq!(p.y, 20.0);
        
        // Also test through setters
        p.set_x(100.0);
        p.set_y(200.0);
        
        assert_eq!(p.x, 100.0);
        assert_eq!(p.y, 200.0);
    }

    #[test]
    fn test_module_helper_function() {
        let p = Point::new(3.0, 4.0);
        let debug_info = point_debug_info(&p);
        
        // The helper function can access private fields
        assert!(debug_info.contains("x=3.0"));
        assert!(debug_info.contains("y=4.0"));
        assert!(debug_info.contains("distance_to_origin=5.0"));
    }

    #[test]
    fn test_conversions_with_direct_access() {
        let value = 7.5;
        let p: Point = value.into();
        
        // Conversion uses direct field access
        assert_eq!(p.x, 7.5);
        assert_eq!(p.y, 7.5);
        
        // Cross-type comparison uses direct field access
        assert!(p == value);
    }

    #[test]
    fn test_public_getters_still_work() {
        let p = Point::new(9.5, 8.5);
        
        // Public getters still exist for external access
        assert_eq!(p.x(), 9.5);
        assert_eq!(p.y(), 8.5);
        
        // But we can also access directly within module
        assert_eq!(p.x, 9.5);
        assert_eq!(p.y, 8.5);
    }
}
