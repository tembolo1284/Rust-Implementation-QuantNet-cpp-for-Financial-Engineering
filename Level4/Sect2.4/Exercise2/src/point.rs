// Point class with Display trait implementation
// ==============================================
// In C++: ostream& operator << (ostream& os, const Point& p);
// In Rust: impl std::fmt::Display for Point
//
// The Display trait is Rust's equivalent of C++'s ostream << operator.
// It allows objects to be formatted with "{}" in print! and format! macros.

use std::fmt;
use std::ops::{Neg, Mul, Add, MulAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

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
    
    // ToString method (equivalent to C++ ToString())
    // This method is used by the Display implementation
    pub fn to_string_custom(&self) -> String {
        format!("Point({:.2}, {:.2})", self.x, self.y)
    }
    
    // ToString with custom precision
    pub fn to_string_precision(&self, precision: usize) -> String {
        format!("Point({:.prec$}, {:.prec$})", self.x, self.y, prec = precision)
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

// Display trait implementation (equivalent to C++ ostream << operator)
// ===================================================================
// In C++:
//   ostream& operator << (ostream& os, const Point& p) {
//       os << p.ToString();  // Using ToString() method
//       return os;
//   }
//
// In Rust, we implement std::fmt::Display:
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the ToString method like in C++ implementation
        if let Some(precision) = f.precision() {
            write!(f, "Point({:.prec$}, {:.prec$})", self.x, self.y, prec = precision)
        } else {
            // Default precision (2 decimal places)
            write!(f, "{}", self.to_string_custom())
        }
    }
}

// Additional formatting traits for more flexibility

// LowerExp for scientific notation (e.g., {:e})
impl fmt::LowerExp for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({:e}, {:e})", self.x, self.y)
    }
}

// UpperExp for scientific notation (e.g., {:E})
impl fmt::UpperExp for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({:E}, {:E})", self.x, self.y)
    }
}

// Binary format (e.g., {:b}) - probably not useful for Point, but demonstrates flexibility
impl fmt::Binary for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point(x_bits: {:064b}, y_bits: {:064b})", 
               self.x.to_bits(), self.y.to_bits())
    }
}

// Operator implementations (from previous exercise)
impl Neg for Point {
    type Output = Point;
    
    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    
    fn mul(self, factor: f64) -> Self::Output {
        Point::new(self.x * factor, self.y * factor)
    }
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_basic() {
        let p = Point::new(3.14159, 2.71828);
        let display = format!("{}", p);
        
        // Should use default precision (2 decimal places)
        assert_eq!(display, "Point(3.14, 2.72)");
    }

    #[test]
    fn test_display_with_precision() {
        let p = Point::new(3.14159, 2.71828);
        
        let low_precision = format!("{:.1}", p);
        let high_precision = format!("{:.4}", p);
        
        assert_eq!(low_precision, "Point(3.1, 2.7)");
        assert_eq!(high_precision, "Point(3.1416, 2.7183)");
    }

    #[test]
    fn test_to_string_methods() {
        let p = Point::new(3.14159, 2.71828);
        
        let default_string = p.to_string_custom();
        let precision_string = p.to_string_precision(4);
        
        assert_eq!(default_string, "Point(3.14, 2.72)");
        assert_eq!(precision_string, "Point(3.1416, 2.7183)");
    }

    #[test]
    fn test_debug_format() {
        let p = Point::new(1.0, 2.0);
        let debug = format!("{:?}", p);
        
        // Debug format should show the struct fields
        assert!(debug.contains("Point"));
        assert!(debug.contains("x: 1.0"));
        assert!(debug.contains("y: 2.0"));
    }

    #[test]
    fn test_scientific_notation() {
        let p = Point::new(1234.567, 0.0001234);
        
        let lower_exp = format!("{:e}", p);
        let upper_exp = format!("{:E}", p);
        
        assert!(lower_exp.contains("e"));
        assert!(upper_exp.contains("E"));
    }

    #[test]
    fn test_binary_format() {
        let p = Point::new(1.0, -1.0);
        let binary = format!("{:b}", p);
        
        assert!(binary.contains("x_bits:"));
        assert!(binary.contains("y_bits:"));
    }

    #[test]
    fn test_format_with_operators() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        
        let sum = p1 + p2;
        let scaled = p1 * 2.0;
        let negated = -p1;
        
        assert_eq!(format!("{}", sum), "Point(4.00, 6.00)");
        assert_eq!(format!("{}", scaled), "Point(2.00, 4.00)");
        assert_eq!(format!("{}", negated), "Point(-1.00, -2.00)");
    }

    #[test]
    fn test_zero_point() {
        let p = Point::default();
        assert_eq!(format!("{}", p), "Point(0.00, 0.00)");
    }

    #[test]
    fn test_negative_coordinates() {
        let p = Point::new(-3.5, -2.1);
        assert_eq!(format!("{}", p), "Point(-3.50, -2.10)");
    }

    #[test]
    fn test_format_in_collections() {
        let points = vec![
            Point::new(1.0, 2.0),
            Point::new(3.0, 4.0),
        ];
        
        let formatted: Vec<String> = points.iter()
            .map(|p| format!("{}", p))
            .collect();
        
        assert_eq!(formatted[0], "Point(1.00, 2.00)");
        assert_eq!(formatted[1], "Point(3.00, 4.00)");
    }
}
